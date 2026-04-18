#![allow(unused)]

use embedded_hal::digital::OutputPin;
use statig::prelude::*;

use fugit::ExtU64;
use crate::ticker::Ticker;
use crate::timer::Timer;

use stm32f1xx_hal::gpio::{Pin, Output, PushPull};
use rtt_target::rprintln;

// #[derive(Default)]
pub struct Blinky<P: OutputPin>
{
    pub timer: Timer,
    pub led: P
}

impl<P: OutputPin> Blinky<P> {
    pub fn new(led: P) -> Self {
        Self {
            timer: Timer::new(1000.millis()),
            led,
        }
    }
}

pub enum Event {
    TimerElapsed
}

#[state_machine(initial = "State::led_on()")]
impl <P: OutputPin> Blinky<P> {
    #[state(entry_action = "enter_led_on", exit_action = "exit_led_on")]
    fn led_on(event: &Event) -> Outcome<State> {
        match event {
            Event::TimerElapsed => Transition(State::led_off()),
            _ => Super
        }
    }

    #[state(entry_action = "enter_led_off", exit_action = "exit_led_off")]
    fn led_off(event: &Event) -> Outcome<State> {
        match event {
            Event::TimerElapsed => Transition(State::led_on()),
            _ => Super
        }
    }

    #[action]
    fn enter_led_on(&mut self) {
        // rprintln!("LED ON");
        self.timer = Timer::new(1000.millis());
        self.led.set_high();
    }

    #[action]
    fn exit_led_on(&mut self) {
        // rprintln!("Switching to LED Off");
    }

    #[action]
    fn enter_led_off(&mut self) {
        // rprintln!("LED OFF");
        self.timer = Timer::new(1000.millis());
        self.led.set_low();
    }

    #[action]
    fn exit_led_off(&mut self) {
        // rprintln!("Switching to LED On");
    }
}

pub fn blinky_poll<P: OutputPin>(blinky_task: &mut InitializedStateMachine<Blinky<P>>) {
    if blinky_task.timer.is_ready() {
        // let time = Ticker::now();
        // rprintln!("Blinky Event triggered at {} ticks, {} ms", time.ticks(), time.duration_since_epoch().to_millis());
        blinky_task.handle(&Event::TimerElapsed);
    }
}
