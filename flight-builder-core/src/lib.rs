//#![no_std]

extern crate core;
extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::cell::{Ref, RefCell, RefMut};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, PartialEq)]
pub enum Schedule{
    Startup,
    
    /// How often to run this task in seconds
    Update(f32),
}

impl Schedule {
    pub fn get_time(&self) -> f32 {
        match self {
            Schedule::Startup => 0.0,
            Schedule::Update(time) => *time,
        }
    }
    
}

pub trait Stage {
    fn invoke(&mut self, args: &mut BTreeMap<TypeId, RefCell<Box<dyn Any>>>);
}

pub type StoredStage = Box<dyn Stage>;

trait IntoStage<Input>{
    type Stage: Stage;
    fn into_stage(self) -> Self::Stage;
}

trait StageParam {
    type Item<'new>;
    fn retrieve<'r>(resources: &'r BTreeMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'r>;
}

pub struct Res<'a, T: 'static> {
    value: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a T>,
}

pub struct ResMut<'a, T: 'static> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}

impl<T> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

impl<T> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

impl<T> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.downcast_mut().unwrap()
    }
}

impl<'res, T: 'static> StageParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn retrieve<'r>(resources: &'r BTreeMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'r> {
        Res { value: resources.get(&TypeId::of::<T>()).unwrap().borrow(), _marker: PhantomData }
    }
}

impl<'res, T: 'static> StageParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn retrieve<'r>(resources: &'r BTreeMap<TypeId,RefCell<Box<(dyn Any + 'static)>>>) -> Self::Item<'r> {
        ResMut {
            value: resources.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_stage {
    ($($params:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F: FnMut($($params),*), $($params: StageParam),*> Stage for FunctionStage<($($params),*), F>
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) +
                FnMut($(<$params as StageParam>::Item<'b>),*)
        {
            fn invoke(&mut self, resources: &mut BTreeMap<TypeId, RefCell<Box<dyn Any>>>) {
                fn call_inner<$($params),*>(
                        mut f: impl FnMut($($params),*),
                    $(
                        $params: $params,
                    )*
                ) {
                    f($($params),*)
                }

                $(
                    let $params = $params::retrieve(resources);
                )*

                call_inner(&mut self.f, $($params),*)
            }
        }

        impl<F: FnMut($($params),*), $($params: StageParam),*> IntoStage<($($params,)*)> for F
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) +
                FnMut($(<$params as StageParam>::Item<'b>),*)
        {
            type Stage = FunctionStage<($($params),*), Self>;

            fn into_stage(self) -> Self::Stage {
                FunctionStage{
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    };
}

impl_stage!();
impl_stage!(A);
impl_stage!(A, B);
impl_stage!(A, B, C);
impl_stage!(A, B, C, D);
impl_stage!(A, B, C, D, E);
impl_stage!(A, B, C, D, E, G);
impl_stage!(A, B, C, D, E, G, H);
impl_stage!(A, B, C, D, E, G, H, I);
impl_stage!(A, B, C, D, E, G, H, I, J);
impl_stage!(A, B, C, D, E, G, H, I, J, K);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M, N);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M, N, O);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P);
impl_stage!(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P, R);

pub struct Scheduler {
    startup_stages: Vec<StoredStage>,
    update_stages: Vec<(u128,u128,StoredStage)>,

    resources: BTreeMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler{
            startup_stages: vec![],
            update_stages: vec![],
            resources: BTreeMap::new(),
        }
    }

    pub fn add_task<I, S: Stage + 'static>(&mut self, schedule: Schedule, stage: impl IntoStage<I, Stage = S>) {
        match schedule {
            Schedule::Startup => {
                self.startup_stages.push(Box::new(stage.into_stage()));
            }
            Schedule::Update(x) => {
                self.update_stages.push((0,(x * 1000.0) as u128,Box::new(stage.into_stage())));
            }
        }
    }

    pub fn add_plugin(&mut self, plugin: impl Fn(&mut Scheduler)){
        plugin(self);
    }

    pub fn add_resource<R: 'static>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), RefCell::new(Box::new(resource)));
    }

    pub fn setup(&mut self) {
        for stage in self.startup_stages.iter_mut() {
            stage.invoke(&mut self.resources);
        }
    }

    pub fn run(&mut self) {
        loop {
            let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            for stage in self.update_stages.iter_mut() {
                if t - stage.0 > stage.1 {
                    stage.2.invoke(&mut self.resources);
                    
                    stage.0 = t;
                }
            }
        }
    }
}

pub struct FunctionStage<Input, F>{
    f: F,
    marker: PhantomData<fn() -> Input>,
}