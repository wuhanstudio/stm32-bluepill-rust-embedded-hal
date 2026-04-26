use cortex_m_rt::exception;
use cortex_m::peripheral::syst::SystClkSource;
use stm32f1xx_hal::rcc;

use core::task::Waker;
use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

use fugit::{Duration, Instant};

use heapless::Vec;
use critical_section::Mutex;

const MAX_TIMERS: usize = 8;

pub struct TimerEntry {
    pub deadline: TickInstant,
    pub waker: Waker,
}

pub static TIMERS: Mutex<RefCell<Vec<TimerEntry, MAX_TIMERS>>> =
    Mutex::new(RefCell::new(Vec::new()));

pub type TickInstant = Instant<u64, 1, 1_000>; // 1 kHz clock
pub type TickDuration = Duration<u64, 1, 1_000>; // 1 kHz clock

static TICKS: AtomicU32 = AtomicU32::new(0);

pub struct Ticker;

impl Ticker {
    pub fn init(syst: &mut cortex_m::peripheral::SYST, clocks: &rcc::Clocks) {
        // Set up SysTick to generate an interrupt every 1 ms
        syst.set_reload(clocks.sysclk().raw() / 1_000 - 1);

        // Use the core clock as the source for SysTick
        syst.set_clock_source(SystClkSource::Core);

        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();
    }

    pub fn now() -> TickInstant {
        TickInstant::from_ticks(TICKS.load(Ordering::Relaxed) as u64)
    }
}

#[exception]
fn SysTick() {
    // This runs every 1ms automatically
    TICKS.fetch_add(1, Ordering::Relaxed);

    let now = Ticker::now();

    critical_section::with(|cs| {
        let timers = &mut *TIMERS.borrow_ref_mut(cs);

        let mut i = 0;
        while i < timers.len() {
            if now >= timers[i].deadline {
                timers[i].waker.wake_by_ref();
                timers.swap_remove(i);
            } else {
                i += 1;
            }
        }
    });
}
