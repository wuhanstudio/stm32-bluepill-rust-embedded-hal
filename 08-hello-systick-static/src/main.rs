#![no_main]
#![no_std]

use panic_halt as _;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

use fugit::ExtU64;

pub mod ticker;
pub mod timer;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    
    // Set up the system clock. We want to run at 48MHz for this one.
    let mut flash = dp.FLASH.constrain();

    let rcc = dp.RCC.freeze(
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
    let mut timer = timer::Timer::new(1000u64.millis());

    rprintln!("System clock: {} Hz", clocks.sysclk().raw());
    rprintln!("SysTick reload: {}", syst.rvr.read());

    let mut i = 1;
    loop {
        // let current = cortex_m::peripheral::SYST::get_current();
        // rprintln!("SysTick current: {}", current);

        if timer.is_ready() {
            rprintln!("[{}] Hello, world! Time: {}", i, ticker::Ticker::now());
            timer = timer::Timer::new(1000u64.millis());

            i = i + 1;
        }
    }
}
