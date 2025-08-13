pub mod startup;
pub mod per_a_tick;
pub mod update;

use crate::scheduler::{Scheduler, Task, IntoTask};

pub use per_a_tick::PerATick;
pub use update::Update;
pub use startup::Startup;

pub trait Schedule<I, T: Task + 'static> {
    fn schedule_task(&self, s: &mut Scheduler, task: impl IntoTask<I, Task = T>);
}
