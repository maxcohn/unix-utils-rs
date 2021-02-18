use std::env;
use std::process::exit;
use std::thread;
use std::time;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        eprintln!("sleep: missing operand");
        exit(1);
    }

    let sleep_time: f64 = args.get(0).unwrap().parse().unwrap();//unwrap().parse().unwrap();

    let dur = time::Duration::from_secs_f64(sleep_time);

    thread::sleep(dur);
}
