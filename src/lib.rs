#![no_std]

pub mod events;
pub mod query;
pub mod scheduler;
pub mod states;
pub mod tasks;

use static_alloc::Bump;

mod clock;
mod map;

pub const MAX_RESOURCES: usize = 1024;
pub const MAX_TASKS: usize = 1024;
pub const MAX_EVENTS: usize = 1024;
pub const MAX_STATES: usize = 1024;
pub const MAX_TRANSITIONS: usize = 1024;
pub const MAX_EVENTS_QUEUE: usize = 1024;
pub const MAX_STARTUP_TASKS: usize = 1024;
pub const MAX_TRANSITION_TASKS: usize = 32;

pub static SLAB: Bump<[u8; 1024]> = Bump::uninit();

extern crate core;

pub mod prelude {
    pub use crate::events::*;
    pub use crate::query::*;
    pub use crate::scheduler::*;
    pub use crate::states::*;
    pub use crate::tasks::*;
    pub use flight_builder_macros::*;
}
