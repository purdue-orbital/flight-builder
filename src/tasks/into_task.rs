use crate::tasks::Task;

pub trait IntoTask<Input> {
    type Task: Task;
    fn into_task(self) -> Self::Task;
}
