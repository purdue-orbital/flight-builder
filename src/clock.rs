use embedded_time::Clock;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::time::Instant;

pub struct SystemClock {
    #[cfg(feature = "std")]
    start: Instant,
}

#[cfg(feature = "std")]
impl Default for SystemClock {
    fn default() -> Self {
        SystemClock {
            start: Instant::now(),
        }
    }
}

#[cfg(feature = "std")]
impl Clock for SystemClock {
    type T = u64;
    const SCALING_FACTOR: embedded_time::rate::Fraction =
        embedded_time::rate::Fraction::new(1, 1_000);

    fn try_now(&self) -> Result<embedded_time::Instant<Self>, embedded_time::clock::Error> {
        Ok(embedded_time::Instant::new(
            (Instant::now() - self.start).as_millis() as u64,
        ))
    }
}
