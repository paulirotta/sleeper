#[cfg(target = "armv7-unknown-linux-gnueabihf")]
extern crate rppal;

mod gpio;

fn main() {
    println!("Hello, sleeper!");
    gpio::print_hello();

    println!("Bye");
}
