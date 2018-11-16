// External crate includes
extern crate rand;

// External crate use's
use rand::distributions::Alphanumeric;
use rand::prelude::*;

// Standard Library use's
use std::env::args;

fn main() {
    let length = args()
        .skip(1)
        .next()
        .expect("Usage: keygen <length>")
        .parse::<usize>()
        .expect("Length must be a positive number.");
    let mut rng = thread_rng();
    let key: String = Alphanumeric
        .sample_iter(&mut rng)
        .filter(|chr: &char| (chr.is_alphabetic() && chr.is_uppercase()) || chr == &'0')
        .take(length)
        .map(|num: char| match num {
            '0' => ' ',
            _ => num,
        })
        .collect();
    println!("{}", key);
}
