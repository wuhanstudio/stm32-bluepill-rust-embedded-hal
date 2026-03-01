#![no_main]
#![no_std]

use panic_halt as _;

// Debugging via RTT, no serrial port needed
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

use statig::prelude::*;

pub mod ticker;
pub mod timer;
pub mod blinky;

use crate::ticker::Ticker;
use crate::blinky::Blinky;
use crate::blinky::blinky_poll;

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
    
    rprintln!("System clock: {} Hz", clocks.sysclk().raw());
    rprintln!("SysTick reload: {}", syst.rvr.read());
    
    let mut blinky_task: InitializedStateMachine<Blinky> = Blinky::default().uninitialized_state_machine().init();
    rprintln!("Waiting for events at {} ms", Ticker::now().duration_since_epoch().to_millis());
    loop {
        blinky_poll(&mut blinky_task);
    }
}
