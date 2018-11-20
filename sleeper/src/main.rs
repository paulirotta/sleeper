#[cfg(feature = "gpio")]
extern crate rppal;

mod gpio;

fn main() {
    println!("Hello, sleeper!");
    gpio::print_hello();

    println!("Bye");
}
