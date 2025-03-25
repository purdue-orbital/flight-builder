#![no_std]
extern crate proc_macro;

pub mod query;
pub mod scheduler;
pub mod states;
pub mod tasks;

mod clock;

extern crate alloc;
extern crate core;

pub mod prelude {
    pub use crate::query::*;
    pub use crate::scheduler::*;
    pub use crate::states::*;
    pub use crate::tasks::*;
    pub use flight_builder_macros::States;
}
