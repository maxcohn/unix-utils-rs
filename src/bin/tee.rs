use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

//TODO: add flags, like -a
fn main() {
    let input = BufReader::new(stdin());

    let files: Vec<String> = env::args().skip(1).collect();

    let mut writers = Vec::new();

    for file in files {
        //TODO: when adding -a option, swap file::create
        writers.push(BufWriter::new(File::create(file).unwrap()));
    }

    let mut output = BufWriter::new(stdout());

    for line in input.lines() {
        let line = line.unwrap();
        for w in &mut writers {
            write!(w, "{}\n", line);
        }
        write!(output, "{}\n", line);
    }
}
