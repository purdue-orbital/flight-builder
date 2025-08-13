pub mod reader;
pub mod writer;

use core::any::Any;
use core::cell::RefCell;
use core::any::TypeId;
use crate::map::Map as HashMap;
use crate::scheduler::MAX_RESOURCES;
use alloc::boxed::Box;


pub trait Event {}

pub struct RegisteredEvent {
    pub(super) id: TypeId,
    pub(super) update: fn(&HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>, TypeId),
}
