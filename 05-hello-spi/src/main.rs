#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use embassy_stm32::Config;
use embassy_stm32::time::Hertz;

// use ssd1306::I2CDisplayInterface;
use embassy_stm32::{gpio, spi::Spi};

// SPI
// MOSI - PA7
// MISO - PA6
// SCK  - PA5
// CS   - PA4

// DC   - PA3
// RST  - PA2

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

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Configure and initialize the embassy STM32 HAL
    let mut config: Config = Default::default();
    config.rcc.hse = Some(embassy_stm32::rcc::Hse {
        freq: Hertz::mhz(8),
        mode: embassy_stm32::rcc::HseMode::Oscillator,
    });

    config.rcc.sys = embassy_stm32::rcc::Sysclk::PLL1_P;
    config.rcc.pll = Some(embassy_stm32::rcc::Pll {
        src: embassy_stm32::rcc::PllSource::HSE,
        prediv: embassy_stm32::rcc::PllPreDiv::DIV1,
        mul: embassy_stm32::rcc::PllMul::MUL9, // 8 * 9 = 72Mhz
    });

    // Scale down to 36Mhz (maximum allowed)
    config.rcc.apb1_pre = embassy_stm32::rcc::APBPrescaler::DIV2;
    let p = embassy_stm32::init(config);

    // Choice 1: Set up the I2C interface
    // let i2c = embassy_stm32::i2c::I2c::new_blocking(
    //     p.I2C1,
    //     p.PB6,
    //     p.PB7,
    //     Default::default(),
    // );

    // let interface = I2CDisplayInterface::new(i2c);
    // let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    //     .into_buffered_graphics_mode();

    // Choice 2: Set up the SPI interface
    let spi = Spi::new_txonly(p.SPI1, p.PA5, p.PA7, p.DMA1_CH3, Default::default());
    let cs = gpio::Output::new(p.PA4, gpio::Level::Low, gpio::Speed::Low);
    let spi = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let mut rst = gpio::Output::new(p.PA2, gpio::Level::Low, gpio::Speed::Low);
    let dc = gpio::Output::new(p.PA3, gpio::Level::Low, gpio::Speed::Low);

    // Initialize the display
    let interface = SPIInterface::new(spi, dc);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display
        .reset(&mut rst, &mut embassy_time::Delay {})
        .unwrap();

    // Initialize the display
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
