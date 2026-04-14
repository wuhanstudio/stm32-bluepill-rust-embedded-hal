#![no_main]
#![no_std]

use panic_halt as _;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

// use statig::prelude::*;

pub mod ticker;
pub mod timer;
pub mod blinky;

use crate::ticker::Ticker;
use crate::timer::delay;

use crate::blinky::Blinky;
use crate::blinky::blinky_poll;

use statig::prelude::*;
use cassette::Cassette;
use fugit::ExtU64;

use core::pin::Pin;
use core::task::Poll;
use core::task::Context;
use core::sync::atomic::{AtomicU32, Ordering};

static TICKS: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy)]
struct CountFuture;

impl Future for CountFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
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
    let count = CountFuture;
    loop {
        count.await;
        rprintln!("[task_1] Hello Count {}", TICKS.load(Ordering::Relaxed));
    }
}

async fn task_2() {
    loop {
        delay(100u64.millis()).await;
        rprintln!("[task_2] Hello World");
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

    let mut blinky_task: InitializedStateMachine<Blinky<_>> = Blinky::new(led).uninitialized_state_machine().init();
    rprintln!("Waiting for events at {} ms", Ticker::now().duration_since_epoch().to_millis());

    let t1 = core::pin::pin!(task_1());
    let mut cm = Cassette::new(t1);
    
    let t2 = core::pin::pin!(task_2());
    let mut cm2 = Cassette::new(t2);

    loop {
        cm.poll_on();
        cm2.poll_on();
        blinky_poll(&mut blinky_task);
    }
}
