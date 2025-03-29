use flight_builder::prelude::*;

#[derive(Event, Debug)]
pub struct TestEvent(pub u32);

fn trigger_event(mut e: ResMut<EventWriter<TestEvent>>) {
    e.send(TestEvent(1));
}

fn print_events(e: Res<EventReader<TestEvent>>) {
    for event in e.iter() {
        println!("Event: {:?}", event);
    }
}

#[test]
pub fn test_event() {
    let mut s = Scheduler::new();

    s.add_event::<TestEvent>();

    s.add_task(Startup, trigger_event);
    s.add_task(Startup, trigger_event);
    s.add_task(Startup, trigger_event);
    s.add_task(Startup, trigger_event);
    s.add_task(Startup, trigger_event);

    s.add_task(Update(0.0), print_events);

    let mut r = s.build();

    r.run_once();
}
