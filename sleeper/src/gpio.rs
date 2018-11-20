#[cfg(target = "arm-unknown-linux-gnueabihf")]
use core::time::Duration;
#[cfg(target = "arm-unknown-linux-gnueabihf")]
use rppal::pwm::*;
#[cfg(target = "arm-unknown-linux-gnueabihf")]
use std::thread::sleep;

#[cfg(not(target = "arm-unknown-linux-gnueabihf"))]
pub fn print_hello() {
    println!("No PWM available unless you are on a Raspberry Pi");
}

#[cfg(target = "arm-unknown-linux-gnueabihf")]
pub fn print_hello() {
    println!("Hello, PWM!");

    if cfg!(target = "arm-unknown-linux-gnueabihf") {
        let period = Duration::from_micros(10000);
        let duty_cycle = Duration::from_micros(10000);

        let throttle =
            rppal::pwm::Pwm::with_period(Channel::Pwm0, period, duty_cycle, Polarity::Normal, true)
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
}
