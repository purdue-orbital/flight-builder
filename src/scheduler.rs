use super::clock::*;
use super::events::*;
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

pub trait Schedule<I, T: Task + 'static> {
    fn schedule_task(&self, s: &mut Scheduler, task: impl IntoTask<I, Task = T>);
}

pub struct Startup;
impl<I, T: Task + 'static> Schedule<I, T> for Startup {
    fn schedule_task(&self, s: &mut Scheduler, task: impl IntoTask<I, Task = T>) {
        s.startup_tasks.push(Box::new(task.into_task()));
    }
}

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

pub struct Scheduler {
    pub(crate) startup_tasks: Vec<StoredTask>,
    pub(crate) update_tasks: Option<Vec<(Microseconds<u64>, Microseconds<u64>, StoredTask)>>,

    pub(crate) resources: Option<HashMap<TypeId, RefCell<Box<dyn Any>>>>,
    //pub(crate) transitions: Option<HashMap<TypeId, RefCell<Transition>>>,
    // pub(crate) states_with_transitions: Option<Vec<TypeId>>,
    //
    pub(crate) registered_events: Option<Vec<RegisteredEvent>>,
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
            //transitions: Some(HashMap::new()),
            //states_with_transitions: Some(vec![]),
            registered_events: Some(vec![]),
        }
    }

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

    pub fn add_event<E: 'static + Event>(&mut self) {
        self.add_resource(EventWriter::<E> { queue: vec![] });
        self.add_resource(EventReader::<E> { queue: vec![] });

        self.registered_events
            .as_mut()
            .unwrap()
            .push(RegisteredEvent {
                id: TypeId::of::<EventWriter<E>>(),
                update: |rec, id| {
                    let mut writer = rec.get(&id).unwrap().borrow_mut();
                    let writer = writer.downcast_mut::<EventWriter<E>>().unwrap();

                    writer.send_to_reader(&rec);
                },
            });
    }

    pub fn build_with_clock<const CLOCK: u32>(&mut self) -> TaskRunner<CLOCK> {
        for task in self.startup_tasks.iter_mut() {
            task.invoke(self.resources.as_mut().unwrap());
        }

        let clock = SystemClock::default();
        self.add_resource(SystemClock::<CLOCK>::default());

        TaskRunner {
            tasks: self.update_tasks.take().unwrap(),
            resources: self.resources.take().unwrap(),

            registered_events: self.registered_events.take().unwrap(),

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
    //states_with_transitions: Vec<TypeId>,
    //transitions: Option<HashMap<TypeId, RefCell<Transition>>>,
    registered_events: Vec<RegisteredEvent>,
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
        for event in self.registered_events.iter() {
            (event.update)(&self.resources, event.id);
        }

        let t = (self.clock.try_now().expect("Error getting current time") - self.start_timestamp)
            .try_into()
            .expect("Failed to convert time");

        for task in self.tasks.iter_mut() {
            if t - task.0 >= task.1 {
                task.2.invoke(&mut self.resources);

                task.0 = t;
            }
        }
    }
}
