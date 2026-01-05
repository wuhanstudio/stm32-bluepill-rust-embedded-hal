#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::*;

use hal::i2c::I2c;
use stm32f1xx_hal as hal;

// SPI
// MOSI - P0.13
// MISO - P0.01
// SCK  - P0.17
// CS   - P1.02

// DC   - P0.09
// RST  - P0.10

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;

use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{Ssd1306, prelude::*};
use ssd1306::I2CDisplayInterface;

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

    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        hal::i2c::Mode::standard(100.kHz()),
        &mut rcc,
    );

    // Initialize the display
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Rust", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
    let im = Image::new(&raw, Point::new(32, 0));
    im.draw(&mut display).unwrap();

    display.flush().unwrap();

    // Wait for the timer to trigger an update and change the state of the LED
    loop {}
}
