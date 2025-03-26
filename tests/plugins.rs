use flight_builder::prelude::*;

pub fn TestPlugin(s: &mut Scheduler) {
    s.add_resource(32u32);
}

#[test]
pub fn test_plugin() {
    let mut s = Scheduler::new();

    s.add_plugin(TestPlugin);
}
