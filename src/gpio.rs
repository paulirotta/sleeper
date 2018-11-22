#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
use core::time::Duration;
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
use rppal::pwm::*;
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
use std::thread::sleep;

#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
pub fn print_hello() {
    println!("Not a Raspberry Pi");
}

#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
pub fn print_hello() {
    println!("Hello, Pi!");

    let period = Duration::from_micros(10000);
    let duty_cycle = Duration::from_micros(10000);

    let throttle = rppal::pwm::Pwm::with_period(Channel::Pwm0, period, duty_cycle, Polarity::Normal, true)
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
