#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;

use stm32f1xx_hal::prelude::*;

// Debugging via RTT, no serrial port needed
use rtt_target::rprintln;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = stm32f1xx_hal::pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    // Set up the system clock. We want to run at 48MHz for this one.
    // let mut flash = dp.FLASH.constrain();
    // let rcc = dp.RCC.freeze(
    //     stm32f1xx_hal::rcc::Config::hse(8.MHz()).sysclk(48.MHz()),
    //     &mut flash.acr,
    // );

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        rprintln!("Hello, world!");
        delay.delay_ms(1000_u32);
    }
}
