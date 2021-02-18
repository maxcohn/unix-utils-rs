use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write};

fn uniq<R: Read, W: Write>(input: R, output: W) {
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let mut line_iter = reader.lines();
    let mut prev_line = line_iter.next().unwrap().unwrap();
    
    write!(&mut writer, "{}\n", prev_line).expect("Failed to write");

    for line in line_iter {
        let line = match line {
            Ok(l) => l,
            Err(e) => panic!(e),
        };

        if line == prev_line {
            continue;
        }

        write!(&mut writer, "{}\n", line).expect("Failed to write");
        prev_line = line;
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        uniq(stdin(), stdout());
    } else if args.len() == 1 {
        let filename = args.get(0).unwrap();

        if filename == "-" {
            uniq(stdin(), stdout());
            return;
        }

        let file = File::open(filename).unwrap();
        uniq(file, stdout());
    } else {
        let outfilename = args.last().unwrap();

        for filename in args.iter().take(args.len() - 1) {
            // we create the writer here because we'll have to pass owership on
            // every call to uniq()
            let writer: Box<dyn Write> = if outfilename == "-" {
                Box::new(stdout())
            } else {
                Box::new(File::create(outfilename).unwrap())
            };

            let reader: Box<dyn Read> = if filename == "-" {
                Box::new(stdin())
            } else {
                Box::new(File::open(filename).unwrap())
            };

            uniq(reader, writer);
        }
    }
}
