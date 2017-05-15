extern crate golang;

use golang::Make;

use std::fmt::Display;

fn print_vec<T>(v : Vec<T>) where T : Display {
    for u in v {
        println!("{}", u);
    }
}

fn main() {
    let mut v : Vec<i32> = Make::make();
    v.push(4);
    v.push(8);
    v.push(15);
    v.push(16);
    v.push(23);
    v.push(42);
    print_vec(v);
}
