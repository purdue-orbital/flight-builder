use embedded_time::duration::Microseconds;

use crate::scheduler::{IntoTask, Scheduler, Task};
use crate::scheduler::schedule::Schedule;
use alloc::boxed::Box;

pub struct PerATick;
impl<I, T: Task + 'static> Schedule<I, T> for PerATick {
    fn schedule_task(&self, s: &mut Scheduler, task: impl IntoTask<I, Task = T>) {
        s.update_tasks
            .as_mut()
            .expect("Task Runner Already Made")
            .push((
                Microseconds::new(0),
                Microseconds::new(0),
                Box::new(task.into_task()),
            ));
    }
}
