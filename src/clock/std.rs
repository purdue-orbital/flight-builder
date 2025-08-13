extern crate std;

use std::time::Instant;
use embedded_time::Clock;

pub struct SystemClock<const CLOCK: u32> {

    start: Instant,
}

impl<const CLOCK: u32> Default for SystemClock<CLOCK> {
    fn default() -> SystemClock<CLOCK> {
        SystemClock {
            start: Instant::now(),
        }
    }
}

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
