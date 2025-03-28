use super::clock::*;
use super::states::*;
use super::tasks::*;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::cell::RefCell;
use embedded_time::Clock;
use embedded_time::Instant;
use embedded_time::duration::Microseconds;
use hashbrown::HashMap;

pub enum Schedule {
    Startup,

    /// How often to run this task in seconds
    Update(f32),
}

pub struct Scheduler {
    startup_tasks: Vec<StoredTask>,
    update_tasks: Option<Vec<(Microseconds<u64>, Microseconds<u64>, StoredTask)>>,

    resources: Option<HashMap<TypeId, RefCell<Box<dyn Any>>>>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            startup_tasks: vec![],
            update_tasks: Some(vec![]),
            resources: Some(HashMap::new()),
        }
    }

    /// Adds a task to the scheduler based on the provided schedule and task. This function takes two generic parameters: I for the input type of the task, and S for the concrete Task implementation that implements the Task trait. The Schedule parameter determines when the task will be run - either at startup or on update (with a given frequency in seconds).
    ///
    /// This function checks if the scheduler has already been initialized, then adds the task to the appropriate list (startup tasks or update tasks) and updates the internal state accordingly.
    pub fn add_task<I, S: Task + 'static>(
        &mut self,
        schedule: Schedule,
        task: impl IntoTask<I, Task = S>,
    ) {
        match schedule {
            Schedule::Startup => {
                self.startup_tasks.push(Box::new(task.into_task()));
            }
            Schedule::Update(x) => {
                self.update_tasks
                    .as_mut()
                    .expect("Task Runner Already Made")
                    .push((
                        Microseconds::new(0),
                        Microseconds::new((x * 1_000_000.0) as u64),
                        Box::new(task.into_task()),
                    ));
            }
        }
    }

    /// Adds a plugin to the scheduler.
    ///
    /// # Parameters
    ///
    /// * `plugin` - A function that takes a mutable reference to the Scheduler and modifies it as needed.
    pub fn add_plugin(&mut self, plugin: impl Fn(&mut Scheduler)) {
        plugin(self);
    }

    /// Adds a resource to the scheduler.
    ///
    /// # Parameters
    ///
    /// * `resource` - The resource to be added, which must have a static lifetime.
    pub fn add_resource<R: 'static>(&mut self, resource: R) {
        self.resources
            .as_mut()
            .expect("Task Runner Already Made")
            .insert(TypeId::of::<R>(), RefCell::new(Box::new(resource)));
    }

    pub fn add_state<S: 'static + States>(&mut self, state: S) {
        self.add_resource(NextState::<S>::Unchanged);
        self.add_resource(State(state));
    }

    pub fn init_state<S: 'static + States + Default>(&mut self) {
        self.add_state(S::default());
    }

    pub fn build_with_clock<const CLOCK: u32>(&mut self) -> TaskRunner<CLOCK> {
        for task in self.startup_tasks.iter_mut() {
            task.invoke(self.resources.as_mut().unwrap());
        }

        let clock = SystemClock::default();

        TaskRunner {
            tasks: self.update_tasks.take().unwrap(),
            resources: self.resources.take().unwrap(),

            start_timestamp: clock.try_now().expect("Error can't start timestamp"),
            clock,
        }
    }

    /// Builds a `TaskRunner` from the scheduler by invoking all startup tasks and preparing update tasks.
    ///
    /// # Parameters
    ///
    /// * `self` - A mutable reference to the `Scheduler` instance.
    ///
    /// # Returns
    ///
    /// * A `TaskRunner` that can execute scheduled tasks based on their timing.
    #[cfg(feature = "std")]
    pub fn build(&mut self) -> TaskRunner<0> {
        self.build_with_clock()
    }
}

pub struct TaskRunner<const CLOCK: u32> {
    tasks: Vec<(Microseconds<u64>, Microseconds<u64>, StoredTask)>,
    resources: HashMap<TypeId, RefCell<Box<dyn Any>>>,

    clock: SystemClock<CLOCK>,
    start_timestamp: Instant<SystemClock<CLOCK>>,
}

impl<const CLOCK: u32> TaskRunner<CLOCK> {
    /// Runs the scheduled tasks in a loop.
    ///
    /// This function continuously checks and runs tasks based on their scheduling criteria:
    /// - It calculates the current time since the UNIX epoch in milliseconds.
    /// - For each task, it checks if the elapsed time since the last execution is greater than the task's specified interval.
    /// - If the condition is met, it invokes the task with the available resources and updates the last executed time to the current time.
    pub fn run(&mut self) -> ! {
        loop {
            self.run_once();
        }
    }

    pub fn run_once(&mut self) {
        let t = (self.clock.try_now().expect("Error getting current time") - self.start_timestamp)
            .try_into()
            .expect("Failed to convert time");

        for task in self.tasks.iter_mut() {
            if t - task.0 > task.1 {
                task.2.invoke(&mut self.resources);

                task.0 = t;
            }
        }
    }
}
