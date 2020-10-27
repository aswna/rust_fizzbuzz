extern crate rust_fizzbuzz;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    rust_fizzbuzz::fizzbuzz(args[1].parse::<i32>().unwrap());
}
