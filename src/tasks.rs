use super::MAX_RESOURCES;
use super::map::Map as HashMap;
use super::query::*;
use core::any::{Any, TypeId};
use core::cell::RefCell;
use core::marker::PhantomData;
use without_alloc::Box;

pub trait Task {
    fn invoke(&mut self, args: &HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>);
}

pub type StoredTask = Box<'static, dyn Task>;

pub struct FunctionTask<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

pub trait IntoTask<Input> {
    type Task: Task;
    fn into_task(self) -> Self::Task;
}

pub(crate) trait TaskParam {
    type Item<'new>;
    fn retrieve<'r>(
        resources: &'r HashMap<TypeId, RefCell<Box<'static, dyn Any>>, MAX_RESOURCES>,
    ) -> Self::Item<'r>;
}

impl<'res, T: 'static> TaskParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn retrieve<'r>(
        resources: &'r HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>,
    ) -> Self::Item<'r> {
        Res {
            value: resources.get(&TypeId::of::<T>()).unwrap().borrow(),
            _marker: PhantomData,
        }
    }
}

impl<'res, T: 'static> TaskParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn retrieve<'r>(
        resources: &'r HashMap<TypeId, RefCell<Box<(dyn Any + 'static)>>, MAX_RESOURCES>,
    ) -> Self::Item<'r> {
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
            fn invoke(&mut self, resources: &HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>) {
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
