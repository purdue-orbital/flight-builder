use core::any::Any;
use core::cell::RefCell;
use core::marker::PhantomData;
use core::any::TypeId;
use alloc::boxed::Box;
use crate::map::Map as HashMap;
use crate::scheduler::MAX_RESOURCES;
use super::TaskParam;
use crate::query::Res;


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
