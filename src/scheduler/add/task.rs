use super::super::Scheduler;
use crate::scheduler::schedule::{Schedule};
use crate::tasks::{IntoTask, Task};

impl Scheduler {

    /// Adds a task to the scheduler based on the provided schedule and task. This function takes two generic parameters: I for the input type of the task, and S for the concrete Task implementation that implements the Task trait. The Schedule parameter determines when the task will be run - either at startup or on update (with a given frequency in seconds).
    ///
    /// This function checks if the scheduler has already been initialized, then adds the task to the appropriate list (startup tasks or update tasks) and updates the internal state accordingly.
    pub fn add_task<I, T: Task + 'static>(
        &mut self,
        schedule: impl Schedule<I, T>,
        task: impl IntoTask<I, Task = T>,
    ) {
        schedule.schedule_task(self, task);
    }
}
