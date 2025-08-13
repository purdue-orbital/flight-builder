use crate::{clock::SystemClock, prelude::TaskRunner};
use embedded_time::Clock;

use super::Scheduler;


impl Scheduler {
    pub fn build_with_clock<const CLOCK: u32>(&mut self) -> TaskRunner<CLOCK> {
        for task in self.startup_tasks.iter_mut() {
            task.invoke(&self.resources.as_ref().unwrap());
        }

        let clock = SystemClock::default();
        let start = clock.try_now().expect("Error can't start clock");
        self.add_resource(clock);

        TaskRunner {
            tasks: self.update_tasks.take().unwrap(),
            resources: self.resources.take().unwrap(),

            registered_events: self.registered_events.take().unwrap(),
            registered_states: self.registered_states.take().unwrap(),
            registered_transitions: self.registered_transitions.take().unwrap(),

            start_timestamp: start,
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
