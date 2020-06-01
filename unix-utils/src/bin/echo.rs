use std::env;

fn main() {
    for (i, arg) in env::args().skip(1).enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
}
