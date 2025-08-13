use crate::scheduler::{IntoTask, Scheduler, Schedule};
use crate::tasks::Task;
use embedded_time::duration::Microseconds;
use alloc::boxed::Box;

pub struct Update(pub f64);
impl<I, T: Task + 'static> Schedule<I, T> for Update {
    fn schedule_task(&self, s: &mut Scheduler, task: impl IntoTask<I, Task = T>) {
        s.update_tasks
            .as_mut()
            .expect("Task Runner Already Made")
            .push((
                Microseconds::new(0),
                Microseconds::new((self.0 * 1_000_000.0) as u64),
                Box::new(task.into_task()),
            ));
    }
}
