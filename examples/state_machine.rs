use flight_builder::prelude::*;

#[derive(States, Default, PartialEq, Eq, Hash, Clone)]
pub enum FunStates {
    #[default]
    State1,
    State2,
    State3,
    State4,
}

pub fn main() {
    let mut s = Scheduler::new();

    s.init_state::<FunStates>();

    s.build().run();
}
