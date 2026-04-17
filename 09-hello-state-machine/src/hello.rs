#![allow(unused)]

use statig::prelude::*;

use fugit::ExtU64;
use crate::ticker::Ticker;
use crate::timer::Timer;

use stm32f1xx_hal::serial::Serial;
use stm32f1xx_hal::pac::USART1;

type MyUart = stm32f1xx_hal::serial::Tx<USART1>;

use core::fmt::Write;

// #[derive(Default)]
pub struct Hello
{
    timer: Timer,
    uart: MyUart,
    count: u32,
}

impl Hello {
    pub fn new(uart: MyUart) -> Self {
        Self {
            timer: Timer::new(2000.millis()),
            uart: uart,
            count: 0,
        }
    }
}

pub enum Event {
    TimerElapsed
}

#[state_machine(initial = "State::print()")]
impl Hello {
    #[state(entry_action = "enter_print")]
    fn print(event: &Event) -> Outcome<State> {
        match event {
            Event::TimerElapsed => Transition(State::idle()),
            _ => Super
        }
    }

    #[state(entry_action = "enter_idle")]
    fn idle(event: &Event) -> Outcome<State> {
        match event {
            Event::TimerElapsed => Transition(State::print()),
            _ => Super
        }
    }

    #[action]
    fn enter_print(&mut self) {
        write!(self.uart, "Hello, World {}\r\n", self.count).unwrap();
        self.count = self.count + 1;
        self.timer = Timer::new(2000.millis());
    }

    #[action]
    fn enter_idle(&mut self) {

    }
}

pub fn hello_poll(hello_task: &mut InitializedStateMachine<Hello>) {
    if hello_task.timer.is_ready() {
        let time = Ticker::now();
        hello_task.handle(&Event::TimerElapsed);
    }
}
