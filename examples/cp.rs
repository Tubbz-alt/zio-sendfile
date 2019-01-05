extern crate zio_sendfile;

use std::env::args;
use std::fs::File;
use zio_sendfile::copy;

fn main() {
    let mut files = args().skip(1);
    match (files.next(), files.next()) {
        (Some(first), Some(second)) => {
            let mut first = File::open(first).expect("failed to open first file");
            let mut second = File::create(second).expect("failed to create second file");

            let result = copy(&mut first, &mut second);
            println!("{:?}", result);
        }
        _ => {
            eprintln!("requires two path arguments")
        }
    }
}
