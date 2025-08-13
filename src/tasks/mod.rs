use super::map::Map as HashMap;
use super::scheduler::MAX_RESOURCES;
use alloc::boxed::Box;
use core::any::{Any, TypeId};
use core::cell::RefCell;

pub mod task;
pub mod stored_task;
pub mod task_param;
pub mod into_task;
pub mod function_task;

use task_param::TaskParam;
pub use stored_task::StoredTask;
use function_task::FunctionTask;
pub use into_task::IntoTask;
pub use task::Task;

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
