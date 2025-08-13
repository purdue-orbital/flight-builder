use cortex_m::peripheral::SYST;
use cortex_m::interrupt::{Mutex, free};
use defmt::println;
use core::cell::Cell;
use core::sync::atomic::{AtomicU32, Ordering};
use embedded_time::Clock;

static MS_LOWER_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static MS_UPPER_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

pub struct SystemClock<const CLOCK: u32> {
    syst: cortex_m::peripheral::SYST,
}

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
