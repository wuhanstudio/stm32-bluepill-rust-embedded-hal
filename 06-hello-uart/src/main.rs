#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

use stm32f1xx_hal::serial::{Config, Serial};

use core::fmt::Write;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    // Set up the system clock. We want to run at 48MHz for this one.
    let mut rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()).sysclk(48.MHz()), &mut flash.acr);

    // Prepare the GPIOA peripheral
    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // USART1
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // Prepare the GPIOB peripheral
    // let mut gpiob = p.GPIOB.split(&mut rcc);

    // USART3
    // Configure pb10 as a push_pull output, this will be the tx pin
    // let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // let rx = gpiob.pb11;

    // Set up the usart device. Take ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),
        Config::default().baudrate(115200.bps()),
        &mut rcc,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, _rx) = serial.split();

    // Create a delay abstraction based on general-pupose 32-bit timer TIM2
    let mut delay = dp.TIM2.delay_us(&mut rcc);

    loop {
        rprintln!("Hello, world!");
        writeln!(tx, "Hello World").unwrap();

        delay.delay_ms(1000_u32);
    }
}
