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
    pub use crate::events::reader::EventReader;
    pub use crate::events::writer::EventWriter;
    pub use crate::scheduler::runner::TaskRunner;
    pub use crate::scheduler::schedule::{Schedule, Startup, Update, PerATick};
    pub use crate::scheduler::{Scheduler, MAX_RESOURCES};
    pub use crate::states::{State, States, transition::{OnEnter, OnExit, OnTransition}};
    pub use crate::tasks::{Task, StoredTask};
    pub use crate::query::{Res, ResMut};
    pub use crate::events::{Event, RegisteredEvent};
    pub use crate::states::{NextState};
    pub use flight_builder_macros::{Event, States};
}
