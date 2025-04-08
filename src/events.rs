use super::map::Map as HashMap;
use super::scheduler::MAX_RESOURCES;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::{Any, TypeId};
use core::cell::RefCell;

pub trait Event {}

pub struct RegisteredEvent {
    pub(super) id: TypeId,
    pub(super) update: fn(&HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>, TypeId),
}

pub struct EventReader<S: Event> {
    pub(crate) queue: Vec<S>,
}

impl<E: Event> EventReader<E> {
    pub fn iter(&self) -> alloc::slice::Iter<'_, E> {
        self.queue.iter()
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct EventWriter<S: Event> {
    pub(crate) queue: Vec<S>,
}

impl<S: Event + 'static> EventWriter<S> {
    pub fn send(&mut self, event: S) {
        self.queue.push(event);
    }

    pub(crate) fn send_to_reader(
        &mut self,
        rescouce: &HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>,
    ) {
        let queue = core::mem::replace(&mut self.queue, Vec::new());

        let mut reader = rescouce
            .get(&TypeId::of::<EventReader<S>>())
            .unwrap()
            .borrow_mut();

        let reader = reader.downcast_mut::<EventReader<S>>().unwrap();

        reader.queue = queue;
    }
}
