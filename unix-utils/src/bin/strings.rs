use std::io::Read;
use std::fs::File;
use std::env;


const BUFFER_SIZE: usize = 1024 * 1024;

fn is_printable(c: u8) -> bool {
    (c >= 0x20 && c <= 0x7e) || c == 0x09
}

fn main() {
    let min_len = 4;
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1]).expect("Couldn't open the file.");

    // buffer to hold the current string of printable characters
    let mut cur_str: [u8; 256] = [0; 256];
    let mut str_counter = 0;

    // file reading buffer
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    // begin reading from the file
    let mut bytes_read = file.read(&mut buffer).unwrap();

    // while we still have data in the file
    while bytes_read > 0 {
        for b in buffer.iter() {
            // if the character is printable, add it to our counter
            if is_printable(*b) {
                cur_str[str_counter] = *b;
                str_counter += 1;
                
            } else {
                // if the character isn't printable, check if we found a long
                // enough string to print
                if str_counter >= min_len {
                    for i in 0..str_counter {
                        print!("{}", cur_str[i] as char)
                    }
                    println!();
                }

                // reset counter
                str_counter = 0;
            }
            
        }
        // refill the buffer with next bytes
        bytes_read = file.read(&mut buffer).unwrap();
    }

}
