use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opt {

    #[structopt(short)]
    all: bool,
    
    #[structopt(short)]
    long_listing: bool,

    files: Vec<String>,
}

fn main() {
    let mut opts = Opt::parse();
    println!("{:?}", opts);

    if opts.files.len() == 0 {
        opts.files.push(String::from("."));
    }
    println!("{:?}", opts);

    for arg in &opts.files {
        let meta = fs::metadata(&arg).expect(format!("Couldn't access file '{}'.", arg).as_str());

        // if the argument is a file, print the name
        if meta.is_file() {
            println!("{}", arg);
            continue;
        }
        // if the argument is a directory, print it's contents
        print_dir(&arg, &opts);
    }
}

fn print_dir(dir: &str, opts: &Opt) {
    // if the argument is a directory, print it's contents
    let dir_contents = std::fs::read_dir(dir).expect("Can't access the current directory.");

    for entry in dir_contents {
        let entry = entry.expect("Couldn't read a file in the directory.");

        let filename = entry.file_name().into_string().unwrap();

        // check if the file is hidden
        if !opts.all && filename.chars().nth(0) == Some('.') {
            continue;
        }

        let meta = entry.metadata().unwrap();

        // print the file/directory
        if meta.is_dir() {
            println!("{}/", filename);
        } else {
            println!("{}", filename);
        }
    }
}
