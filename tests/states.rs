use flight_builder::prelude::*;

#[derive(Default, States, Debug, PartialEq, Eq)]
pub enum TestStates {
    #[default]
    Test1,
    Test2,
}

fn set_test1(mut s: ResMut<NextState<TestStates>>) {
    s.set(TestStates::Test1);
}

fn set_test2(mut s: ResMut<NextState<TestStates>>) {
    s.set(TestStates::Test2);
}

fn reset_state(mut s: ResMut<NextState<TestStates>>) {
    s.reset();
}

fn assert_none(mut s: ResMut<NextState<TestStates>>) {
    assert_eq!(None, s.take_next_state())
}

fn assert_pending_test2(mut s: ResMut<NextState<TestStates>>) {
    assert_eq!(
        TestStates::Test2,
        s.take_next_state().expect("Expected a pending next state")
    )
}

fn assert_test1(s: Res<State<TestStates>>) {
    assert_eq!(TestStates::Test1, *s.get())
}

fn assert_test2(s: Res<State<TestStates>>) {
    assert_eq!(TestStates::Test2, *s.get())
}

#[test]
pub fn test_add_state() {
    let mut s = Scheduler::new();

    s.add_state(TestStates::Test2);

    s.add_task(Update(0.0), assert_test2);

    let mut r = s.build();

    r.run_once();
}

#[test]
pub fn test_init_state() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();

    s.add_task(Update(0.0), assert_test1);

    let mut r = s.build();

    r.run_once();
}

#[test]
pub fn test_next_state() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();

    s.add_task(Update(0.0), assert_none);

    s.add_task(Update(0.0), set_test2);

    s.add_task(Update(0.0), assert_pending_test2);

    s.add_task(Update(0.0), reset_state);

    let mut r = s.build();

    r.run_once();
}
