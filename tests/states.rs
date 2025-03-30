use std::{thread::sleep, time::Duration};

use flight_builder::prelude::*;

#[derive(Default, States, Debug, PartialEq, Clone)]
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

    s.build().run_once();
}

#[test]
pub fn test_init_state() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();
    s.add_task(Update(0.0), assert_test1);

    s.build().run_once();
}

#[test]
pub fn test_next_state() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();

    s.add_task(Update(0.0), assert_none);
    s.add_task(Update(0.0), set_test2);
    s.add_task(Update(0.0), assert_pending_test2);
    s.add_task(Update(0.0), reset_state);

    s.build().run_once();
}

#[test]
pub fn test_transition() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();
    s.add_task(Startup, set_test1);
    s.add_task(OnEnter(TestStates::Test1), set_test2);
    s.add_task(Update(0.001), assert_test2);

    let mut r = s.build();

    r.run_once();

    sleep(Duration::from_secs_f32(0.01));

    r.run_once();
}

#[test]
pub fn test_transition_on_exit() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();
    s.add_task(Startup, set_test2);
    s.add_task(OnExit(TestStates::Test1), set_test1);
    s.add_task(Update(0.001), assert_test1);

    let mut r = s.build();

    r.run_once();

    sleep(Duration::from_secs_f32(0.1));

    r.run_once();
}

#[test]
pub fn test_transition_on_transition() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();
    s.add_task(Startup, set_test2);
    s.add_task(
        OnTransition(TestStates::Test1, TestStates::Test2),
        set_test1,
    );
    s.add_task(Update(0.001), assert_test1);

    let mut r = s.build();

    r.run_once();

    sleep(Duration::from_secs_f32(0.1));

    r.run_once();
}

#[test]
pub fn test_multi_on_enters() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();
    s.add_task(Startup, set_test1);
    s.add_task(OnEnter(TestStates::Test1), set_test2);
    s.add_task(OnEnter(TestStates::Test1), set_test2);
    s.add_task(Update(0.001), assert_test2);

    let mut r = s.build();

    r.run_once();

    sleep(Duration::from_secs_f32(0.1));

    r.run_once();
}

#[test]
pub fn test_multi_on_exits() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();
    s.add_task(Startup, set_test2);
    s.add_task(OnExit(TestStates::Test1), set_test1);
    s.add_task(OnExit(TestStates::Test1), set_test1);
    s.add_task(Update(0.001), assert_test1);

    let mut r = s.build();

    r.run_once();

    sleep(Duration::from_secs_f32(0.1));

    r.run_once();
}

#[test]
pub fn test_multi_on_transitions() {
    let mut s = Scheduler::new();

    s.init_state::<TestStates>();
    s.add_task(Startup, set_test2);
    s.add_task(
        OnTransition(TestStates::Test1, TestStates::Test2),
        set_test1,
    );
    s.add_task(
        OnTransition(TestStates::Test1, TestStates::Test2),
        set_test1,
    );
    s.add_task(Update(0.001), assert_test1);

    let mut r = s.build();

    r.run_once();

    sleep(Duration::from_secs_f32(0.1));

    r.run_once();
}
