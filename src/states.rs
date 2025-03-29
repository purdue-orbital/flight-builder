use super::scheduler::Schedule;
use super::tasks::Task;
use alloc::boxed::Box;
use alloc::vec::Vec;

use super::tasks::StoredTask;

// pub struct OnEnter<S: States + 'static>(S);
// impl<S: States + 'static, I, T: Task + 'static> Schedule<I, T> for OnEnter<S> {
//     fn schedule_task(
//         &self,
//         s: &mut crate::prelude::Scheduler,
//         task: impl crate::prelude::IntoTask<I, Task = T>,
//     ) {
//         let map = s.transitions.as_mut().expect("Task Runner already made!");

//         // Check if the transition exists
//         if let Some(transition) = map.get_mut(&core::any::TypeId::of::<S>()) {
//             transition
//                 .borrow_mut()
//                 .on_enter
//                 .push(Box::new(task.into_task()));
//         } else {
//             let mut transition = crate::prelude::Transition {
//                 on_enter: Vec::new(),
//                 on_exit: Vec::new(),
//             };

//             transition.on_enter.push(Box::new(task.into_task()));

//             map.insert(
//                 core::any::TypeId::of::<S>(),
//                 core::cell::RefCell::new(transition),
//             );
//         }
//     }
// }

pub struct OnExit<S: States + 'static>(S);

pub trait States {}

pub struct Transition {
    pub(crate) on_enter: Vec<StoredTask>,
    pub(crate) on_exit: Vec<StoredTask>,
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
