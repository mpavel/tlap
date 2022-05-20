#![allow(
dead_code,
unused_imports
)]

use ch2::luhn_checksum;
use ch2::triangle;

mod ch2;

fn main() {
    println!("Starting ... ");

    // ch2
    // triangle::run();
    luhn_checksum::run();
}
