use super::super::Scheduler;
use crate::events::reader::EventReader;
use crate::events::writer::EventWriter;
use crate::states::state_transition_event::StateTransitionEvent;
use crate::states::{NextState, RegisteredState, RegisteredTransition, State, States};
use core::any::TypeId;

impl Scheduler {


    pub fn init_state<S: 'static + States + Default + Clone + PartialEq>(&mut self) {
        self.add_state(S::default());
    }


    pub fn add_state<S: 'static + States + Clone + PartialEq>(&mut self, state: S) {
        self.add_resource(NextState::<S>::Unchanged);
        self.add_resource(State(state));

        self.add_event::<StateTransitionEvent<S>>();

        self.registered_states
            .as_mut()
            .unwrap()
            .push(RegisteredState {
                id: TypeId::of::<State<S>>(),
                event_id: TypeId::of::<EventWriter<StateTransitionEvent<S>>>(),
                update: |rec, id, event_id| {
                    let mut state = rec.get(&id).unwrap().borrow_mut();
                    let state = state.downcast_mut::<State<S>>().unwrap();

                    let mut next_state =
                        rec.get(&TypeId::of::<NextState<S>>()).unwrap().borrow_mut();
                    let next_state = next_state.downcast_mut::<NextState<S>>().unwrap();

                    let mut event_writer = rec.get(&event_id).unwrap().borrow_mut();
                    let event_writer = event_writer
                        .downcast_mut::<EventWriter<StateTransitionEvent<S>>>()
                        .unwrap();

                    if let Some(next_state) = next_state.take_next_state() {
                        let last = state.get().clone();
                        state.0 = next_state.clone();

                        event_writer.send(StateTransitionEvent {
                            from: last,
                            to: next_state.clone(),
                        });
                    }
                },
            });

        self.registered_transitions
            .as_mut()
            .unwrap()
            .push(RegisteredTransition {
                id: TypeId::of::<EventReader<StateTransitionEvent<S>>>(),
                update: |rec, id| {
                    let event_reader = rec.get(&id).unwrap().borrow();
                    let event_reader = event_reader
                        .downcast_ref::<EventReader<StateTransitionEvent<S>>>()
                        .unwrap();

                    for event in event_reader.queue.iter() {
                        event.apply_transition(&rec);
                    }
                },
            });
    }
}
