use super::super::Scheduler;
impl Scheduler {
    /// Adds a plugin to the scheduler.
    ///
    /// # Parameters
    ///
    /// * `plugin` - A function that takes a mutable reference to the Scheduler and modifies it as needed.
    pub fn add_plugin(&mut self, plugin: impl Fn(&mut Scheduler)) {
        plugin(self);
    }

}
