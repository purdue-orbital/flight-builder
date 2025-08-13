use core::any::TypeId;
use alloc::boxed::Box;
use crate::scheduler::{Scheduler};
use crate::states::{States, transition::Transition};
use crate::tasks::Task;
use crate::tasks::IntoTask;
use crate::scheduler::schedule::{Schedule};

use alloc::vec;

pub struct OnTransition<S: States>(pub S, pub S);


impl<I, T: Task + 'static, S: States + 'static + PartialEq + Clone> Schedule<I, T>
    for OnTransition<S>
{
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
