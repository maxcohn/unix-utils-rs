use std::fs;
use std::env;

fn main() {
    let args = env::args().skip(1).collect();
    println!("Hello, world!");
}
