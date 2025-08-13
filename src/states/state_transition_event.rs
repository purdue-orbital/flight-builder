use core::any::Any;
use core::cell::RefCell;
use core::any::TypeId;
use alloc::boxed::Box;
use crate::map::Map as HashMap;
use crate::scheduler::MAX_RESOURCES;
use crate::states::{States, transition::Transition};
use flight_builder_macros::{Event};
use crate::events::Event;

#[derive(Event)]
pub struct StateTransitionEvent<S: States> {
    pub from: S,
    pub to: S,
}

impl<S: States + 'static + PartialEq> StateTransitionEvent<S> {
    pub(crate) fn apply_transition(
        &self,
        r: &HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>,
    ) {
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
