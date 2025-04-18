#![no_std]

pub mod events;
pub mod query;
pub mod scheduler;
pub mod states;
pub mod tasks;

mod map;

mod clock;

extern crate alloc;
extern crate core;

pub mod prelude {
    pub use crate::events::*;
    pub use crate::query::*;
    pub use crate::scheduler::*;
    pub use crate::states::*;
    pub use crate::tasks::*;
    pub use flight_builder_macros::*;
}
