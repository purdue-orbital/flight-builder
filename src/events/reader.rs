use super::Event;
use alloc::vec::Vec;


pub struct EventReader<S: Event> {
    pub(crate) queue: Vec<S>,
}

impl<E: Event> EventReader<E> {
    pub fn iter(&self) -> alloc::slice::Iter<'_, E> {
        self.queue.iter()
    }
}
