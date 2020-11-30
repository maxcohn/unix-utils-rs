use std::fs;

use clap::{App, AppSettings, Arg};

struct CmdOptions {
    all: bool,

    //TODO: make use of -l option at some point. currently metadata.permissions() is limited in what it shows
    long_listing: bool,
}

fn main() {
    let (opts, mut inputs) = parse_cli();

    if inputs.len() == 0 {
        inputs.push(String::from("."));
    }

    for arg in inputs {
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

fn parse_cli() -> (CmdOptions, Vec<String>) {
    let matches = App::new("ls")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Shows hidden files"),
        )
        .arg(
            Arg::with_name("long listing")
                .short("l")
                .help("Use a long listing format"),
        )
        .arg(Arg::with_name("inputs").multiple(true).required(false))
        .get_matches();

    let inputs: Vec<String> = match matches.values_of("inputs") {
        Some(vals) => vals.map(|s| String::from(s)).collect(),
        None => vec![],
    };

    let opts = CmdOptions {
        all: matches.is_present("all"),
        long_listing: matches.is_present("long listing"),
    };

    (opts, inputs)
}

fn print_dir(dir: &str, opts: &CmdOptions) {
    // if the argument is a directory, print it's contents
    let dir_contents = std::fs::read_dir(dir).expect("Can't access the current directory.");

    for entry in dir_contents {
        let entry = entry.expect("Couldn't read a file in the directory.");

        let filename = entry.file_name().into_string().unwrap();

        // check if the file is hidden
        if !opts.all && filename.chars().nth(0).unwrap() == '.' {
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
