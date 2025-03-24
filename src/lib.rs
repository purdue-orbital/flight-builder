//#![no_std]

pub mod query;
pub mod scheduler;
pub mod tasks;

extern crate alloc;
extern crate core;

pub mod prelude {
    pub use crate::query::*;
    pub use crate::scheduler::*;
    pub use crate::tasks::*;
}
