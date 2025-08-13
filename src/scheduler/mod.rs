use super::events::*;
use super::map::Map as HashMap;
use super::states::*;
use super::tasks::*;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::cell::RefCell;
use embedded_time::duration::Microseconds;


pub const MAX_RESOURCES: usize = 1024;

pub mod schedule;
pub mod runner;
pub mod add;
pub mod build;

use crate::scheduler::schedule::{Schedule};

pub struct Scheduler {
    pub(crate) startup_tasks: Vec<StoredTask>,
    pub(crate) update_tasks: Option<Vec<(Microseconds<u64>, Microseconds<u64>, StoredTask)>>,
    pub(crate) resources: Option<HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>>,

    pub(crate) registered_events: Option<Vec<RegisteredEvent>>,
    pub(crate) registered_states: Option<Vec<RegisteredState>>,
    pub(crate) registered_transitions: Option<Vec<RegisteredTransition>>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            startup_tasks: vec![],
            update_tasks: Some(vec![]),
            resources: Some(HashMap::new()),

            registered_events: Some(vec![]),
            registered_states: Some(vec![]),
            registered_transitions: Some(vec![]),
        }
    }
}
