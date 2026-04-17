#![no_main]
#![no_std]

use panic_halt as _;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};
use stm32f1xx_hal::serial::{Config, Serial};

use statig::prelude::*;

pub mod ticker;
pub mod timer;
pub mod blinky;
pub mod hello;

use crate::ticker::Ticker;

use crate::blinky::Blinky;
use crate::blinky::blinky_poll;

use crate::hello::Hello;
use crate::hello::hello_poll;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    
    // Set up the system clock. We want to run at 48MHz for this one.
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
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
    
    rprintln!("System clock: {} Hz", clocks.sysclk().raw());
    rprintln!("SysTick reload: {}", syst.rvr.read());
    
    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut blinky_task: InitializedStateMachine<Blinky<_>> = Blinky::new(led).uninitialized_state_machine().init();
    rprintln!("Waiting for events at {} ms", Ticker::now().duration_since_epoch().to_millis());

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
    let (tx, _rx) = serial.split();

    let mut hello_task: InitializedStateMachine<Hello> = Hello::new(tx).uninitialized_state_machine().init();
    loop {
        blinky_poll(&mut blinky_task);
        hello_poll(&mut hello_task);
    } 
}
