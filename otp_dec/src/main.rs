extern crate rand;

use rand::prelude::*;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process::exit;

fn main() {
    let mut arguments = args().skip(1);

    let mut plaintext = String::new();
    let ciphertext_file = arguments.next().unwrap_or_else(|| {
        eprintln!("Usage: otp_dec <ciphertext_file> <key_file> <port>");
        exit(1);
    });
    File::open(ciphertext_file)
        .unwrap_or_else(|_| {
            eprintln!("Couldn't read plaintext file.");
            exit(2);
        })
        .read_to_string(&mut plaintext)
        .unwrap_or_else(|_| {
            eprintln!("Plaintext file was not well-formatted UTF-8.");
            exit(3);
        });
    let plaintext = plaintext.trim_right();

    let mut key = String::new();
    let filename = arguments.next().unwrap_or_else(|| {
        eprintln!("Usage: otp_dec <ciphertext_file> <key_file> <port>");
        exit(4);
    });
    File::open(filename)
        .unwrap_or_else(|_| {
            eprintln!("Couldn't read key file.");
            exit(5);
        })
        .read_to_string(&mut key)
        .unwrap_or_else(|_| {
            eprintln!("Plaintext file was not well-formatted UTF-8.");
            exit(6);
        });
    let key = key.trim_right();

    let port = arguments
        .next()
        .unwrap_or_else(|| {
            eprintln!("Usage: otp_dec <ciphertext_file> <key_file> <port>");
            exit(6);
        })
        .parse::<u32>()
        .expect("Couldn't parse port into valid u32.");

    let mut stream = TcpStream::connect(&format!("localhost:{}", port)).unwrap_or_else(|_| {
        eprintln!("Couldn't connect to the daemon. Are you sure it's running?");
        exit(7);
    });

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
            .write_all(format!("dec|{}|{}|{}", plaintext, key, response_port).as_bytes())
            .unwrap_or_else(|_| {
                eprintln!("Couldn't send data to the daemon. Is it blocked?");
                exit(8);
            });

        drop(stream);
        response_stream = response_listener
            .accept()
            .unwrap_or_else(|_| {
                eprintln!("Response stream failed to connect.");
                exit(9);
            })
            .0;
        break;
    }

    let mut response = String::new();
    let _ = response_stream.read_to_string(&mut response);
    let parts: Vec<&str> = response.split('|').collect();

    match parts.as_slice() {
        [response, ""] => {
            println!("{}", response);
        },
        ["", error] => {
            eprintln!("{}", error);
        },
        _ => eprintln!("Unexpected result from daemon.")
    }

}
