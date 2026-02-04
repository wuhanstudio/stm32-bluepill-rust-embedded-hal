#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

// use cortex_m::peripheral::syst;
use stm32f1xx_hal::{pac, prelude::*, rcc};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    
    // Set up the system clock. We want to run at 48MHz for this one.
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()).sysclk(48.MHz()), &mut flash.acr);

    let clocks = rcc.clocks;
    rprintln!("System clock: {} Hz", clocks.sysclk().raw());
    
    // 1 ms tick
    let cp: pac::CorePeripherals = cortex_m::Peripherals::take().unwrap();
    let mut syst  = cp.SYST;

    syst.set_reload(clocks.sysclk().raw() / 1_000 - 1);

    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    loop {

    }
}