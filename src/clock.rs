use embedded_time::Clock;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::time::Instant;

#[cfg(feature = "cortex-m")]
use cortex_m::peripheral::SYST;

#[cfg(feature = "cortex-m")]
use cortex_m::interrupt::{Mutex, free};

#[cfg(feature = "cortex-m")]
use defmt::println;

#[cfg(feature = "cortex-m")]
use core::cell::Cell;

#[cfg(feature = "cortex-m")]
use core::sync::atomic::{AtomicU32, Ordering};

#[cfg(feature = "cortex-m")]
static MS_LOWER_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

#[cfg(feature = "cortex-m")]
static MS_UPPER_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

pub struct SystemClock<const CLOCK: u32> {
    #[cfg(feature = "std")]
    start: Instant,

    #[cfg(feature = "cortex-m")]
    syst: cortex_m::peripheral::SYST,
}

#[cfg(feature = "cortex-m")]
#[cortex_m_rt::exception]
fn SysTick() {
    free(|cs| {
        let lower = MS_LOWER_COUNTER.borrow(cs).get();
        MS_LOWER_COUNTER.borrow(cs).set(lower + 1);

        if lower == u32::MAX {
            let upper = MS_UPPER_COUNTER.borrow(cs).get();
            MS_UPPER_COUNTER.borrow(cs).set(upper + 1);
        }
    })
}

#[cfg(feature = "std")]
impl<const CLOCK: u32> Default for SystemClock<CLOCK> {
    fn default() -> SystemClock<CLOCK> {
        SystemClock {
            start: Instant::now(),
        }
    }
}

#[cfg(feature = "cortex-m")]
impl<const CLOCK: u32> Default for SystemClock<CLOCK> {
    fn default() -> SystemClock<CLOCK> {
        let c = cortex_m::Peripherals::take().unwrap();
        let mut syst = c.SYST;

        syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        syst.set_reload(CLOCK / 10_000);
        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();

        SystemClock { syst }
    }
}

#[cfg(feature = "std")]
impl<const CLOCK: u32> Clock for SystemClock<CLOCK> {
    type T = u64;
    const SCALING_FACTOR: embedded_time::rate::Fraction =
        embedded_time::rate::Fraction::new(1, 1_000_000);

    fn try_now(&self) -> Result<embedded_time::Instant<Self>, embedded_time::clock::Error> {
        Ok(embedded_time::Instant::new(
            (Instant::now() - self.start).as_micros() as u64,
        ))
    }
}

#[cfg(feature = "cortex-m")]
impl<const CLOCK: u32> Clock for SystemClock<CLOCK> {
    type T = u64;
    const SCALING_FACTOR: embedded_time::rate::Fraction =
        embedded_time::rate::Fraction::new(1, 10_000); // had to lower to allow time for systick

    fn try_now(&self) -> Result<embedded_time::Instant<Self>, embedded_time::clock::Error> {
        free(|cs| {
            let time = (MS_UPPER_COUNTER.borrow(cs).get() as u64) << 32
                | (MS_LOWER_COUNTER.borrow(cs).get() as u64);

            Ok(embedded_time::Instant::new(time))
        })
    }
}
