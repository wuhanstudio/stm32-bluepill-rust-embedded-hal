#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    // Set up the system clock. We want to run at 48MHz for this one.
    let mut rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()).sysclk(48.MHz()), &mut flash.acr);

    // Create a delay abstraction based on general-pupose 32-bit timer TIM2
    let mut delay = dp.TIM2.delay_us(&mut rcc);

    loop {
        rprintln!("Hello, world!");
        delay.delay_ms(1000_u32);
    }
}
