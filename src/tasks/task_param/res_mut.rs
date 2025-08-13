use core::any::Any;
use core::cell::RefCell;
use core::marker::PhantomData;
use core::any::TypeId;
use alloc::boxed::Box;
use crate::map::Map as HashMap;
use crate::scheduler::MAX_RESOURCES;
use super::TaskParam;
use crate::query::ResMut;

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
