use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    let mut message = String::from("y");
    if args.len() != 0 {
        message = args.join(" ");
    }

    loop {
        println!("{}", message);
    }
}
