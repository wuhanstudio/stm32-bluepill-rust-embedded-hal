#![no_std]
#![no_main]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_time::Timer;
use embassy_executor::Spawner;
use embassy_stm32::gpio;
use embassy_stm32::rcc::*;
use embassy_stm32::Config;

use core::pin::Pin;
use core::task::Poll;
use core::task::Context;
use core::sync::atomic::{AtomicU32, Ordering};

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

#[embassy_executor::task]
async fn task_1() {
    loop {
        let count = CountFuture;
        count.await;
        rprintln!("[task_1] Hello Count {}", TICKS.load(Ordering::Relaxed));
    }
}

#[embassy_executor::task]
async fn task_2() {
    loop {
        rprintln!("(task_2) Hello World");
        Timer::after_millis(2000).await;
    }
}

#[embassy_executor::task]
async fn task_led(mut led: gpio::Output<'static>) {
    loop {
        // LED On
        led.set_low();
        Timer::after_millis(1000).await;

        // LED Off
        led.set_high();
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    rtt_init_print!();

    let mut config = Config::default();

    config.rcc.hse = Some(Hse {
        freq: embassy_stm32::time::Hertz(8_000_000),
        mode: HseMode::Oscillator, 
    });

    config.rcc.pll = Some(Pll {
        src: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL9,
    });

    let p = embassy_stm32::init(config);
    let led = gpio::Output::new(p.PC13, gpio::Level::High, gpio::Speed::Low);

    spawner.spawn(task_1()).unwrap();
    spawner.spawn(task_2()).unwrap();
    spawner.spawn(task_led(led)).unwrap();
}
