use crate::{tasks::{IntoTask, Task}};
use alloc::boxed::Box;
use crate::scheduler::{Scheduler, Schedule};


pub struct Startup;
impl<I, T: Task + 'static> Schedule<I, T> for Startup {
    fn schedule_task(&self, s: &mut Scheduler, task: impl IntoTask<I, Task = T>) {
        s.startup_tasks.push(Box::new(task.into_task()));
    }
}
