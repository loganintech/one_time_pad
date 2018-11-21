extern crate rand;

use rand::prelude::*;

use std::env::args;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut arguments = args().skip(1);

    let plaintext = arguments
        .next()
        .expect("Usage: otp_enc <plaintext> <one_time_pad> <port>");

    let one_time_pad = arguments
        .next()
        .expect("Usage: otp_enc <plaintext> <one_time_pad> <port>");

    let port = arguments
        .next()
        .expect("Usage: otp_enc <plaintext> <one_time_pad> <port>")
        .parse::<u32>()
        .expect("Couldn't parse port into valid u32.");

    let mut stream = TcpStream::connect(&format!("localhost:{}", port))?;

    let mut rng = thread_rng();
    let mut response_port: u32 = (rng.next_u32() % 40_000) + 20_000;

    while let Err(_) = TcpListener::bind(format!("localhost:{}", response_port)) {
        response_port = (rng.next_u32() % 40_000) + 20_000;
    };


    stream.write_all(format!("{}|{}|{}", plaintext, one_time_pad, response_port).as_bytes())?;
    drop(stream);

    let mut response = vec![];
    let _ = stream.read_to_end(&mut response);
    println!("Response: {}", String::from_utf8(response).unwrap());
    Ok(())
}
