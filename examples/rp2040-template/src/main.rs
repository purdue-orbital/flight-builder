// This example shows how to use a global allocator for dynamic memory allocation
// when using no_std

#![no_std]
#![no_main]

use core::ptr::addr_of_mut;
use core::u8;
use embedded_alloc::LlffHeap as Heap;
use flight_builder::prelude::Scheduler;

use embassy_rp::gpio::{Level, Output};
use flight_builder::prelude::*;
use {defmt_rtt as _, panic_probe as _};

extern crate alloc;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn switch_light(mut led: ResMut<Output>) {
    if led.get_output_level() == Level::High {
        led.set_low();
    } else {
        led.set_high()
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1280;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    let p = embassy_rp::init(Default::default());
    let led = Output::new(p.PIN_25, Level::Low);

    let mut s = Scheduler::default();

    s.add_resource(led);

    s.add_task(Schedule::Startup, switch_light);

    s.add_task(Schedule::Update(1.0), switch_light);

    s.build_with_clock::<133_000_000>().run();
}
