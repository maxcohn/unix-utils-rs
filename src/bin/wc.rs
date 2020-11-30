use std::fs::File;
use std::io::{BufReader, Read};

const BUFFER_SIZE: usize = 1024;

fn main() {
    count_file(&std::env::args().nth(1).unwrap());
}

fn count_file(file_path: &str) {
    let f = File::open(file_path).expect(&format!("Failed to open file: '{}'", file_path));

    let mut reader = BufReader::new(f);
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    let mut char_count = 0;
    let mut word_count = 0;
    let mut line_count = 0;

    let mut in_word = false;

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        let contents =
            std::str::from_utf8(&buffer).expect(&format!("Error reading file: {}", file_path));
        println!("{}", contents);
        for (i, c) in contents.chars().enumerate() {
            if i >= bytes_read {
                break;
            }

            char_count += 1;
            if c.is_whitespace() && in_word {
                in_word = false;
                word_count += 1;
            } else if !c.is_whitespace() && !in_word {
                in_word = true;
            }

            if c == '\n' {
                line_count += 1;
            }
        }

        if bytes_read < BUFFER_SIZE {
            break;
        }
    }
    println!("{} {} {}", line_count, word_count, char_count);
}
