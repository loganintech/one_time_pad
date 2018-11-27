extern crate rand;

use rand::prelude::*;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut arguments = args().skip(1);

    let mut ciphertext = String::new();
    let cipher_text_file = arguments
        .next()
        .expect("Usage: otp_dec <ciphertext> <key_file> <port>");
    File::open(cipher_text_file)?.read_to_string(&mut ciphertext)?;
    let ciphertext = ciphertext.trim_right();

    let mut one_time_pad = String::new();
    let filename = arguments
        .next()
        .expect("Usage: otp_dec <plaintext> <key_file> <port>");
    File::open(filename)?.read_to_string(&mut one_time_pad)?;
    let one_time_pad = one_time_pad.trim_right();

    let port = arguments
        .next()
        .expect("Usage: otp_dec <ciphertext> <key_file> <port>")
        .parse::<u32>()
        .expect("Couldn't parse port into valid u32.");

    let mut stream = TcpStream::connect(&format!("localhost:{}", port))?;

    let mut rng = thread_rng();
    let mut response_port: u32 = (rng.next_u32() % 40_000) + 20_000;

    let mut response_stream: TcpStream;

    loop {
        let listener = TcpListener::bind(format!("localhost:{}", response_port));

        let response_listener = match listener {
            Ok(response_listener) => response_listener,
            Err(_) => {
                response_port = (rng.next_u32() % 40_000) + 20_000;
                continue;
            }
        };

        stream
            .write_all(format!("{}|{}|{}", ciphertext, one_time_pad, response_port).as_bytes())?;
        drop(stream);
        response_stream = response_listener.accept()?.0;
        break;
    }

    let mut response = vec![];
    let _ = response_stream.read_to_end(&mut response);
    println!("{}", String::from_utf8(response).unwrap());
    Ok(())
}
