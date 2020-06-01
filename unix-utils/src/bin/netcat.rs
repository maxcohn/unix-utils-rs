use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::thread;
use clap::{App, Arg, AppSettings};

fn main() -> std::io::Result<()> {
    // get command line arguments
    let matches = App::new("netcat")
        .setting(AppSettings::AllowMissingPositional)
        .arg(Arg::with_name("listen")
            .short("l")
            .long("listen"))
        .arg(Arg::with_name("ip"))
        .arg(Arg::with_name("port").required(true))
        .get_matches();
    
    // get user defined IP if exists, else use the default of 0.0.0.0
    let ip = if matches.is_present("ip") {
        matches.value_of("ip").unwrap()
    } else {
        "0.0.0.0"
    };

    let port = matches.value_of("port").unwrap();

    let mut writer;
    if matches.is_present("listen") {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
        let (conn, _) = listener.accept().unwrap();
        writer = conn;
    } else {
        writer = TcpStream::connect(format!("{}:{}", ip, port))?;
    }

    let mut reader = BufReader::new(writer.try_clone()?);

    // create the thread to read from the other end of the connection
    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            print!("{}", buf);
        }
    });

    // read from stdin (line by line) and write it to the TCP stream
    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                writer.write(line.as_bytes())?;
                writer.write(&[b'\n'])?;
            },
            Err(_) => panic!("Error writing to the TCP stream"),
        };
    }
    
    Ok(())
}
