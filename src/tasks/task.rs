use core::any::Any;
use core::any::TypeId;
use core::cell::RefCell;
use crate::map::Map as HashMap;
use crate::scheduler::MAX_RESOURCES;
use alloc::boxed::Box;

pub trait Task {
    fn invoke(&mut self, args: &HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>);
}
