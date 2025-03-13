use flight_builder_core::{ResMut, Schedule, Scheduler};

pub struct PopTimer{
    pub counter: u32,
}

fn pop_check(mut pop_timer: ResMut<PopTimer>) {
    pop_timer.counter += 1;
    
    if pop_timer.counter == 10 {
        pop();
        
        pop_timer.counter = 0;
    }
}

fn pop(){
    println!("Pop!");
}

fn main() {
    let mut s = Scheduler::new();
    
    s.add_resource(PopTimer{counter:0});
    s.add_task(Schedule::Update(1.0), pop_check);
    s.setup();
    
    s.run()
}