extern crate otp_d_lib;

use std::env::args;
use std::net::Ipv4Addr;

fn main() {
    let port = args()
        .skip(1)
        .next()
        .expect("Usage: otp_enc_d <port>")
        .parse::<u32>()
        .expect("Couldn't parse port into valid u32.");

    if port > 65536 {
        eprintln!("The port must be between 1 and 65536 inclusive.");
        return;
    }

    let url = String::from("127.0.0.1");
}
