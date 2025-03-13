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

pub trait Task {
    fn invoke(&mut self, args: &mut BTreeMap<TypeId, RefCell<Box<dyn Any>>>);
}

pub type StoredTask = Box<dyn Task>;

pub trait IntoTask<Input>{
    type Task: Task;
    fn into_task(self) -> Self::Task;
}

trait TaskParam {
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

impl<'res, T: 'static> TaskParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn retrieve<'r>(resources: &'r BTreeMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'r> {
        Res { value: resources.get(&TypeId::of::<T>()).unwrap().borrow(), _marker: PhantomData }
    }
}

impl<'res, T: 'static> TaskParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn retrieve<'r>(resources: &'r BTreeMap<TypeId,RefCell<Box<(dyn Any + 'static)>>>) -> Self::Item<'r> {
        ResMut {
            value: resources.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_task {
    ($($params:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F: FnMut($($params),*), $($params: TaskParam),*> Task for FunctionTask<($($params),*), F>
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) +
                FnMut($(<$params as TaskParam>::Item<'b>),*)
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

        impl<F: FnMut($($params),*), $($params: TaskParam),*> IntoTask<($($params,)*)> for F
        where
            for<'a, 'b> &'a mut F:
                FnMut($($params),*) +
                FnMut($(<$params as TaskParam>::Item<'b>),*)
        {
            type Task = FunctionTask<($($params),*), Self>;

            fn into_task(self) -> Self::Task {
                FunctionTask{
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    };
}

impl_task!();
impl_task!(A);
impl_task!(A, B);
impl_task!(A, B, C);
impl_task!(A, B, C, D);
impl_task!(A, B, C, D, E);
impl_task!(A, B, C, D, E, G);
impl_task!(A, B, C, D, E, G, H);
impl_task!(A, B, C, D, E, G, H, I);
impl_task!(A, B, C, D, E, G, H, I, J);
impl_task!(A, B, C, D, E, G, H, I, J, K);
impl_task!(A, B, C, D, E, G, H, I, J, K, L);
impl_task!(A, B, C, D, E, G, H, I, J, K, L, M);
impl_task!(A, B, C, D, E, G, H, I, J, K, L, M, N);
impl_task!(A, B, C, D, E, G, H, I, J, K, L, M, N, O);
impl_task!(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P);
impl_task!(A, B, C, D, E, G, H, I, J, K, L, M, N, O, P, R);

pub struct Scheduler {
    startup_tasks: Vec<StoredTask>,
    update_tasks: Option<Vec<(u128,u128,StoredTask)>>,

    resources: Option<BTreeMap<TypeId, RefCell<Box<dyn Any>>>>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler{
            startup_tasks: vec![],
            update_tasks: Some(vec![]),
            resources: Some(BTreeMap::new()),
        }
    }

    pub fn add_task<I, S: Task + 'static>(&mut self, schedule: Schedule, task: impl IntoTask<I, Task = S>)  {
        match schedule {
            Schedule::Startup => {
                self.startup_tasks.push(Box::new(task.into_task()));
            }
            Schedule::Update(x) => {
                self.update_tasks.as_mut().expect("Task Runner Already Made").push((0,(x * 1000.0) as u128,Box::new(task.into_task())));
            }
        }
    }

    pub fn add_plugin(&mut self, plugin: impl Fn(&mut Scheduler)){
        plugin(self);
    }

    pub fn add_resource<R: 'static>(&mut self, resource: R) {
        self.resources.as_mut().expect("Task Runner Already Made").insert(TypeId::of::<R>(), RefCell::new(Box::new(resource)));
    }

    pub fn build(&mut self) -> TaskRunner{
        for task in self.startup_tasks.iter_mut() {
            task.invoke(self.resources.as_mut().unwrap());
        }
        
        TaskRunner{
            tasks: self.update_tasks.take().unwrap(),
            resources: self.resources.take().unwrap(),
        }
    }
}

pub struct TaskRunner{
    tasks: Vec<(u128,u128,StoredTask)>,

    resources: BTreeMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl TaskRunner {
    pub fn run(&mut self) {
        loop {
            let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            for task in self.tasks.iter_mut() {
                if t - task.0 > task.1 {
                    task.2.invoke(&mut self.resources);

                    task.0 = t;
                }
            }
        }
    }
}

pub struct FunctionTask<Input, F>{
    f: F,
    marker: PhantomData<fn() -> Input>,
}