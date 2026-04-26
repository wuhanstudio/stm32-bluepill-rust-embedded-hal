#![no_main]
#![no_std]

use panic_halt as _;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, rcc};

pub mod ticker;
pub mod timer;
pub mod executor;

use fugit::ExtU64;

use core::pin::pin;
use core::pin::Pin;
use core::task::Poll;
use core::task::Context;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

use crate::executor::Task;

static TICKS: AtomicU32 = AtomicU32::new(0);

// #[derive(Clone, Copy)]
struct CountFuture;

impl Future for CountFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Not allowed here due to Pin
        // let _moved = *self;

        let x = TICKS.fetch_add(1, Ordering::SeqCst);
        if (x % 30000) == 0 {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn task_1() {
    loop {
        let count = CountFuture;
        count.await;
        rprintln!("[task_1] Hello Count {}", TICKS.load(Ordering::Relaxed));
    }
}

async fn task_2() {
    loop {
        timer::delay(2000u64.millis()).await;
        rprintln!("[task_2] Hello World");
    }
}

async fn task_led<P: OutputPin>(mut led: P) {
    loop {
        // LED On
        led.set_high().unwrap();
        timer::delay(1000u64.millis()).await;

        // LED Off
        led.set_low().unwrap();
        timer::delay(1000u64.millis()).await;
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    
    // Set up the system clock. We want to run at 48MHz for this one.
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::default()
            .use_hse(8.MHz())       // use external 8 MHz crystal
            .sysclk(48.MHz()),      // target 48 MHz system clock
        &mut flash.acr
    );

    let clocks = rcc.clocks;

    // 1 ms tick
    let cp: pac::CorePeripherals = cortex_m::Peripherals::take().unwrap();
    let mut syst  = cp.SYST;

    ticker::Ticker::init(&mut syst, &clocks);
    
    rprintln!("System clock: {} Hz", clocks.sysclk().raw());
    rprintln!("SysTick reload: {}", syst.rvr.read());
    
    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let t1 = Task {
        future: pin!(task_1()),
        ready: AtomicBool::new(true),
    };

    let t2 = Task {
        future: pin!(task_2()),
        ready: AtomicBool::new(true),
    };

    let t3 = Task {
        future: pin!(task_led(led)),
        ready: AtomicBool::new(true),
    };

    executor::run_tasks(&mut [t1, t2, t3]);
}
