use core::any::Any;
use core::cell::RefCell;
use core::any::TypeId;
use crate::map::Map as HashMap;
use crate::scheduler::MAX_RESOURCES;
use alloc::boxed::Box;

pub mod res;
pub mod res_mut;

pub(crate) trait TaskParam {
    type Item<'new>;
    fn retrieve<'r>(
        resources: &'r HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>,
    ) -> Self::Item<'r>;
}
