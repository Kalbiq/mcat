//! My own attempt at making a cat-like tool
//! I only made it in order to learn Rust
//! If you want to give me some advice or propose a better way of doing something
//! just make an issue on the repo. I would be very grateful :D

use std::env;
use std::process;

use mcat::Config;


fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    if let Err(e) = mcat::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
