use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // if no files are specified, read from stdin
    if args.len() == 0 {
        let input = stdin();
        let mut buf = String::new();
        loop {
            match input.read_line(&mut buf) {
                Ok(bytes) => {
                    if bytes == 0 {
                        std::process::exit(0);
                    } else {
                        print!("{}", &buf);
                    }
                }
                Err(_) => panic!("Failed to read line."),
            }
            buf.clear();
        }
    }

    // loop through all files and print them
    for arg in env::args().skip(1) {
        let mut output = stdout();
        let mut buf = [0; 1024];
        let mut file =
            File::open(&arg).expect(format!("Failed to open file: \"{}\"", arg).as_str());
        let mut bytes_read;

        loop {
            bytes_read = file
                .read(&mut buf)
                .expect(format!("Failed to read file: \"{}\"", arg).as_str());

            output.write_all(&buf).unwrap();

            // check if we're done reading the current file
            if bytes_read < buf.len() {
                break;
            }
        }
    }
}
