use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;

use super::tasks::StoredTask;

pub type StoredState = Box<dyn States>;

pub trait States {}

pub struct Transition<S: States> {
    on_exit: Vec<StoredTask>,
    on_enter: Vec<StoredTask>,

    _phantom: PhantomData<S>,
}

pub struct State<S: States>(pub(crate) S);

impl<S: States> State<S> {
    pub fn get(&self) -> &S {
        &self.0
    }
}

#[derive(Default)]
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

    pub(crate) fn take_next_state(&mut self) -> Option<S> {
        match core::mem::take(self) {
            Self::Pending(x) => Some(x),

            Self::Unchanged => None,
        }
    }
}
