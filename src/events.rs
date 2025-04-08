use super::MAX_EVENTS;
use super::MAX_RESOURCES;
use super::map::Map as HashMap;
use core::any::{Any, TypeId};
use core::cell::RefCell;
use without_alloc::Box;

pub trait Event {}

pub struct RegisteredEvent {
    pub(super) id: TypeId,
    pub(super) update: fn(&HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>, TypeId),
}

pub struct EventReader<S: Event> {
    pub(crate) queue: [Option<S>; MAX_EVENTS],
    pub(crate) index: usize,
}

impl<S: Event> Default for EventReader<S> {
    fn default() -> Self {
        EventReader {
            queue: [const { None }; MAX_EVENTS],
            index: 0,
        }
    }
}

impl<S: Event> Iterator for EventReader<S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= MAX_EVENTS {
            return None;
        }

        let event = self.queue[self.index].take();
        self.index += 1;

        event
    }
}

#[derive(PartialEq, Eq)]
pub struct EventWriter<S: Event> {
    pub(crate) queue: [Option<S>; MAX_EVENTS],
    pub(crate) index: usize,
}

impl<S: Event> Default for EventWriter<S> {
    fn default() -> Self {
        EventWriter {
            queue: [const { None }; MAX_EVENTS],
            index: 0,
        }
    }
}

impl<S: Event + 'static> EventWriter<S> {
    pub fn send(&mut self, event: S) {
        if self.index >= MAX_EVENTS {
            panic!("Event queue is full");
        }

        self.queue[self.index] = Some(event);
        self.index += 1;
    }

    pub(crate) fn send_to_reader(
        &mut self,
        rescouce: &HashMap<TypeId, RefCell<Box<dyn Any>>, MAX_RESOURCES>,
    ) {
        let queue = core::mem::replace(&mut self.queue, [const { None }; MAX_EVENTS]);

        let mut reader = rescouce
            .get(&TypeId::of::<EventReader<S>>())
            .unwrap()
            .borrow_mut();

        let reader = reader.downcast_mut::<EventReader<S>>().unwrap();

        reader.queue = queue;
    }
}
