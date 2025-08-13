#[cfg(feature = "cortex-m")]
mod cortex_m;

#[cfg(feature = "std")]
mod std;


#[cfg(feature = "cortex-m")]
pub use cortex_m::SystemClock;

#[cfg(feature = "std")]
pub use std::SystemClock;
