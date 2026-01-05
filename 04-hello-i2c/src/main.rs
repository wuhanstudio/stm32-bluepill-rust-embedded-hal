#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::*;

use hal::i2c::I2c;
use stm32f1xx_hal as hal;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rtt_target::rprintln!("Panic occurred: {}", _info);
    loop {}
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32f1xx_hal::pac::Peripherals::take().unwrap();

    // Acquire the GPIOC peripheral
    let mut rcc = dp.RCC.constrain();

    let gpiob = dp.GPIOB.split(&mut rcc);

    // Configure I2C1
    let scl = gpiob.pb6;
    let sda = gpiob.pb7;

    let mut i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        hal::i2c::Mode::standard(100.kHz()),
        &mut rcc,
    );

    rtt_target::rprintln!("Start i2c scanning...");
    rtt_target::rprintln!("Please wait for scanning to complete...");
    rtt_target::rprintln!();

    // Print header
    for header in 0x0..0x10 {
        rtt_target::rprint!("{:02x} ", header);
    }
    rtt_target::rprintln!();

    for addr in 0x00_u8..0x80 {
        // Write the empty array and check the slave response.
        let byte: [u8; 1] = [0; 1];
        if i2c.write(addr, &byte).is_ok() {
            rtt_target::rprint!("{:02x}", addr);
        } else {
            rtt_target::rprint!("..");
        }
        if addr % 0x10 == 0x0F {
            rtt_target::rprintln!();
        } else {
            rtt_target::rprint!(" ");
        }
    }

    rtt_target::rprintln!();
    rtt_target::rprintln!("Done!");

    // Wait for the timer to trigger an update and change the state of the LED
    loop {}
}
