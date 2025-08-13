use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::cell::RefCell;
use embedded_time::Clock;
use embedded_time::Instant;
use embedded_time::duration::Microseconds;

use crate::clock::SystemClock;
use crate::events::RegisteredEvent;
use crate::states::{RegisteredState, RegisteredTransition};
use crate::tasks::StoredTask;

use crate::scheduler::MAX_RESOURCES;
use crate::map::Map as HashMap;

pub struct TaskRunner<const CLOCK: u32> {
    pub tasks: Vec<(Microseconds<u64>, Microseconds<u64>, StoredTask)>,
    pub resources: HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>,

    pub start_timestamp: Instant<SystemClock<CLOCK>>,

    pub registered_events: Vec<RegisteredEvent>,
    pub registered_states: Vec<RegisteredState>,
    pub registered_transitions: Vec<RegisteredTransition>,
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
        // Update the states
        for state in self.registered_states.iter() {
            (state.update)(&self.resources, state.id, state.event_id);
        }

        // Update the events
        for event in self.registered_events.iter() {
            (event.update)(&self.resources, event.id);
        }

        // Update the transitions
        for transition in self.registered_transitions.iter() {
            (transition.update)(&self.resources, transition.id);
        }

        let t = self
            .resources
            .get(&TypeId::of::<SystemClock<CLOCK>>())
            .unwrap()
            .borrow();

        let clock = t.downcast_ref::<SystemClock<CLOCK>>().unwrap();

        // Get the current time
        let time = (clock.try_now().expect("Error getting current time") - self.start_timestamp)
            .try_into()
            .expect("Failed to convert time");

        drop(t);

        // Run the tasks
        for task in self.tasks.iter_mut() {
            if time - task.0 >= task.1 {
                task.2.invoke(&mut self.resources);

                task.0 = time;
            }
        }
    }
}
