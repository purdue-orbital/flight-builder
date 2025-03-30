# Flight Builder

A rust RTOS library that is designed to work on microcontrollers and systems that implment the standard library.

# Table of Contents
1. [Quickstart](#quickstart)
2. [Flags](#flags)
3. [License](#license)

## Quickstart

```rust
use flight_builder::prelude::*;

pub struct SensorReadings {
    pub bar: f32,
    pub temperature: f32,
    pub humidity: f32,
}

pub fn check_sensors(mut readings: ResMut<SensorReadings>) {
    readings.bar = 1.0;
    readings.temperature = 22.0;
    readings.humidity = 0.4;
}

pub fn print_sensors(readings: Res<SensorReadings>) {
    println!(
        "Bar: {}  Temperature: {} Humidity: {}",
        readings.bar, readings.temperature, readings.humidity
    );
}

fn main() {
    let mut s = Scheduler::default();

    // Add sensor reading resource
    s.add_resource(SensorReadings {
        bar: 0.0,
        temperature: 0.0,
        humidity: 0.0,
    });

    // Add a task that updates sensor readings as fast as the system can run
    s.add_task(PerATick(), check_sensors);

    // Add a task that runs every 5 seconds
    s.add_task(Update(5.0), print_sensors);

    s.build().run();
}
```


## Flags
- `default`: By default this crate implments the std flag. To use in no_std, do add the following to your cargo toml instead `flight-builder = { version = "0.1.0", default-features = false, features = [<instert platform you are on>]` }
- `cortex-m`: This makes this library work on cortex-m based proccessors. Some examples of cortex-m platforms: RP2040, RP2350, Teensey, and STM32 microcontrollers. Be sure that you have configured your memory.x file accordingly. There are some templates and examples in the [examples](examples/) folder if want to use it. Be sure to set default-features = false.

## License
This project is licensed under the [MIT license](LICENSE)
