extern crate otp_lib;

use otp_lib::decode;
use std::env::args;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = vec![];
    if let Err(e) = stream.read_to_end(&mut buffer) {
        eprintln!("Error occured reading stream: {}", e);
        return;
    }

    match String::from_utf8(buffer) {
        Ok(buff) => {
            let parts: Vec<&str> = buff.split('|').collect();
            if parts.len() != 3 {
                let _ = stream.write_all("Incorrectly formatted string.".as_bytes());
                return;
            }
            let cipher = decode(parts[0], parts[1]);
            let mut stream = match TcpStream::connect(format!("localhost:{}", parts[2])) {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("Couldn't connect for response: {}", e);
                    return;
                }
            };
            let _ = stream.write_all(cipher.unwrap_or_else(|e| e).as_bytes());
        },
        Err(e) => {
            eprintln!("Couldn't parse buffer as string: {}", e);
            let _ = stream.write_all("Couldn't parse buffer as string.".as_bytes());
        }
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let port = args()
        .skip(1)
        .next()
        .expect("Usage: otp_enc_d <port>")
        .parse::<u32>()
        .expect("Couldn't parse port into valid u32.");

    if port > 65536 {
        eprintln!("The port must be between 1 and 65536 inclusive.");
        return Ok(());
    }

    let addr = format!("localhost:{}", port);
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream?;

        thread::spawn(move || {
            handle_stream(stream);
        });
    }

    Ok(())
}
