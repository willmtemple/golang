#[macro_use]
extern crate golang;

use std::thread;
use std::time::Duration;

fn main() {
    for i in 1..10 {
        go!(println!("{}", i));
    }
    thread::sleep(Duration::new(1, 0));
    println!("All done!");
}
