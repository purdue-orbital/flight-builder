use super::events::Event;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::cell::RefCell;
use flight_builder_macros::Event;

use super::scheduler::Scheduler;
use super::tasks::{IntoTask, Task};
use crate::events::EventReader;
use crate::prelude::Schedule;
use crate::query::Res;
use crate::tasks::StoredTask;
use hashbrown::HashMap;

pub trait States {}

pub struct RegisteredState {
    pub(super) id: TypeId,
    pub(super) event_id: TypeId,
    pub(super) update: fn(&HashMap<TypeId, RefCell<Box<dyn Any>>>, TypeId, TypeId),
}

pub struct RegisteredTransition {
    pub(super) id: TypeId,
    pub(super) update: fn(&HashMap<TypeId, RefCell<Box<dyn Any>>>, TypeId),
}

#[derive(Event)]
pub struct StateTransitionEvent<S: States> {
    pub from: S,
    pub to: S,
}

impl<S: States + 'static + PartialEq> StateTransitionEvent<S> {
    pub(super) fn apply_transition(&self, r: &HashMap<TypeId, RefCell<Box<dyn Any>>>) {
        let mut transition = r.get(&TypeId::of::<Transition<S>>()).unwrap().borrow_mut();
        let transition = transition.downcast_mut::<Transition<S>>().unwrap();

        // On Exit
        for (state, tasks) in transition.on_exit.iter_mut() {
            if state == &self.from {
                for task in tasks.iter_mut() {
                    task.invoke(r);
                }
            }
        }

        // On Transition
        for (from, to, tasks) in transition.on_transition.iter_mut() {
            if from == &self.from && to == &self.to {
                for task in tasks.iter_mut() {
                    task.invoke(r);
                }
            }
        }

        // On Enter
        for (state, tasks) in transition.on_enter.iter_mut() {
            if state == &self.to {
                for task in tasks.iter_mut() {
                    task.invoke(r);
                }
            }
        }
    }
}

pub struct OnEnter<S: States>(pub S);
pub struct OnExit<S: States>(pub S);
pub struct OnTransition<S: States>(pub S, pub S);

impl<I, T: Task + 'static, S: States + 'static + PartialEq + Clone> Schedule<I, T> for OnEnter<S> {
    fn schedule_task(&self, s: &mut Scheduler, task: impl IntoTask<I, Task = T>) {
        let t = s
            .resources
            .as_mut()
            .unwrap()
            .get_mut(&TypeId::of::<Transition<S>>());

        let t = if t.is_none() {
            s.add_resource(Transition::<S> {
                on_enter: vec![],
                on_exit: vec![],
                on_transition: vec![],
            });

            s.resources
                .as_mut()
                .unwrap()
                .get_mut(&TypeId::of::<Transition<S>>())
        } else {
            t
        };

        let arr = &mut t.unwrap().borrow_mut();
        let arr = arr.downcast_mut::<Transition<S>>().unwrap();
        let arr = &mut arr.on_enter;

        // check if the state is already in the list
        if let Some((_, tasks)) = arr.iter_mut().find(|(state, _)| state == &self.0) {
            tasks.push(Box::new(task.into_task()));
        } else {
            arr.push((self.0.clone(), vec![Box::new(task.into_task())]));
        }
    }
}

impl<I, T: Task + 'static, S: States + 'static + PartialEq + Clone> Schedule<I, T> for OnExit<S> {
    fn schedule_task(&self, s: &mut Scheduler, task: impl IntoTask<I, Task = T>) {
        let t = s
            .resources
            .as_mut()
            .unwrap()
            .get_mut(&TypeId::of::<Transition<S>>())
            .expect("Transition Not Registered");

        let arr = &mut t.borrow_mut();
        let arr = arr.downcast_mut::<Transition<S>>().unwrap();
        let arr = &mut arr.on_exit;

        // check if the state is already in the list
        if let Some((_, tasks)) = arr.iter_mut().find(|(state, _)| state == &self.0) {
            tasks.push(Box::new(task.into_task()));
        } else {
            arr.push((self.0.clone(), vec![Box::new(task.into_task())]));
        }
    }
}

impl<I, T: Task + 'static, S: States + 'static + PartialEq + Clone> Schedule<I, T>
    for OnTransition<S>
{
    fn schedule_task(&self, s: &mut Scheduler, task: impl IntoTask<I, Task = T>) {
        let t = s
            .resources
            .as_mut()
            .unwrap()
            .get_mut(&TypeId::of::<Transition<S>>())
            .expect("Transition Not Registered");

        let arr = &mut t.borrow_mut();
        let arr = arr.downcast_mut::<Transition<S>>().unwrap();
        let arr = &mut arr.on_transition;

        // check if the state is already in the list
        if let Some((_, _, tasks)) = arr
            .iter_mut()
            .find(|(from, to, _)| from == &self.0 && to == &self.1)
        {
            tasks.push(Box::new(task.into_task()));
        } else {
            arr.push((
                self.0.clone(),
                self.1.clone(),
                vec![Box::new(task.into_task())],
            ));
        }
    }
}

pub struct Transition<S: States> {
    pub(super) on_enter: Vec<(S, Vec<StoredTask>)>,
    pub(super) on_exit: Vec<(S, Vec<StoredTask>)>,
    pub(super) on_transition: Vec<(S, S, Vec<StoredTask>)>,
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
