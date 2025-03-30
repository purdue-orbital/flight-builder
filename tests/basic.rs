use std::ops::{Deref, DerefMut};

use flight_builder::prelude::*;

pub fn basic_function() {
    let _ = 1 + 1;
}

pub fn plus_1(mut num: ResMut<u32>) {
    let m = num.deref().clone();
    let i = num.deref_mut();

    assert_eq!(*i, m);

    *i = i.deref() + 1u32;
}

pub fn assert_equal<const N: u32>(num: Res<u32>) {
    assert_eq!(num.deref(), &N)
}

#[test]
pub fn test_make_scheduler() {
    let _ = Scheduler::new();
}

#[test]
pub fn test_make_scheduler_default() {
    let _ = Scheduler::default();
}

#[test]
pub fn test_basic_function() {
    let mut s = Scheduler::new();

    s.add_task(Update(0.0), basic_function);

    s.build().run_once();
}

#[test]
pub fn test_basic_startup() {
    let mut s = Scheduler::new();

    s.add_task(Startup, basic_function);

    s.build().run_once();
}

#[test]
pub fn test_add_resouce() {
    let mut s = Scheduler::new();

    s.add_resource(2u32);

    s.build().run_once();
}

#[test]
pub fn test_query() {
    let mut s = Scheduler::new();

    s.add_resource(2u32);

    s.add_task(Update(0.0), plus_1);
    s.add_task(Update(0.0), assert_equal::<3u32>);

    s.add_task(PerATick, plus_1);
    s.add_task(PerATick, plus_1);
    s.add_task(PerATick, plus_1);
    s.add_task(PerATick, assert_equal::<6u32>);

    let mut r = s.build();

    r.run_once();
}
