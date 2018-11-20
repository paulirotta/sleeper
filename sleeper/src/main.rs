#[cfg(feature = "gpio")]
extern crate rppal;

use core::time::Duration;
#[cfg(feature = "gpio")]
use rppal::pwm::*;
use std::thread::sleep;

fn main() {
    println!("Hello, sleeper!");

    if cfg!(features = "gpio") {
        let period = Duration::from_micros(10000);
        let duty_cycle = Duration::from_micros(10000);

        let throttle = Pwm::with_period(Channel::Pwm0, period, duty_cycle, Polarity::Normal, true)
            .expect("Could not initialize throttle");

        let steering = Pwm::with_period(Channel::Pwm1, period, duty_cycle, Polarity::Normal, true)
            .expect("Could not initialize steering");

        let delay = 2000;
        let delay2 = 1;
        let mut val = 1000;
        sleep(Duration::from_millis(delay));

        for val2 in 0..3000 {
            let d = val2;
            throttle.set_duty_cycle(Duration::from_micros(d));
            steering.set_duty_cycle(Duration::from_micros(d));
            println!("{} micros", d);
            sleep(Duration::from_millis(delay2));
        }
    }

    println!("Bye");
}
