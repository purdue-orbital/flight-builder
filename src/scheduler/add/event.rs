use super::super::Scheduler;
use crate::events::reader::EventReader;
use crate::events::writer::EventWriter;
use crate::events::{Event, RegisteredEvent};
use core::any::TypeId;
use alloc::vec;

impl Scheduler {
    pub fn add_event<E: 'static + Event>(&mut self) {
        self.add_resource(EventReader::<E> { queue: vec![] });
        self.add_resource(EventWriter::<E> { queue: vec![] });

        self.registered_events
            .as_mut()
            .unwrap()
            .push(RegisteredEvent  {
                id: TypeId::of::<EventWriter<E>>(),
                update: |rec, id| {
                    let mut writer = rec.get(&id).unwrap().borrow_mut();
                    let writer = writer.downcast_mut::<EventWriter<E>>().unwrap();

                    writer.send_to_reader(&rec);
                },
            });
    }
}
