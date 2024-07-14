use std::fs::File;
use std::io;
use std::io::prelude::*;

fn log_something(filename: &'static str, string: &'static [u8; 12]) {
    let mut f = File::create(filename).unwrap();
    f.write_all(string);
}

fn main() {
    log_something("log.txt", b"ITS ALIVE!!!")
}
