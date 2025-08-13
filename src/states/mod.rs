use alloc::boxed::Box;
use core::any::{Any, TypeId};
use core::cell::RefCell;

use super::map::Map as HashMap;
use super::scheduler::MAX_RESOURCES;

pub mod transition;
pub mod state_transition_event;

pub trait States {}

pub struct RegisteredState {
    pub(super) id: TypeId,
    pub(super) event_id: TypeId,
    pub(super) update: fn(&HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>, TypeId, TypeId),
}

pub struct RegisteredTransition {
    pub(super) id: TypeId,
    pub(super) update: fn(&HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>, TypeId),
}

pub struct State<S: States>(pub(crate) S);

impl<S: States> State<S> {
    pub fn get(&self) -> &S {
        &self.0
    }
}

#[derive(Default, PartialEq, Eq)]
pub enum NextState<S: States> {
    #[default]
    Unchanged,

    Pending(S),
}

impl<S: States> NextState<S> {
    pub fn set(&mut self, new_state: S) {
        *self = Self::Pending(new_state);
    }

    pub fn reset(&mut self) {
        *self = Self::Unchanged;
    }

    pub fn take_next_state(&mut self) -> Option<S> {
        match core::mem::take(self) {
            Self::Pending(x) => Some(x),

            Self::Unchanged => None,
        }
    }
}
