#![no_main]
#![no_std]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::*;

// Debugging via RTT, no serrial port needed
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32f1xx_hal::pac::Peripherals::take().unwrap();

    // Acquire the GPIOC peripheral
    let mut rcc = dp.RCC.constrain();
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Configure the syst timer to trigger an update every second
    let mut timer = stm32f1xx_hal::timer::Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        defmt::info!("GPIO High");

        block!(timer.wait()).unwrap();
        led.set_low();
        defmt::info!("GPIO LOW");
    }
}
