extern crate zio_sendfile;

use std::env::args;
use std::fs::File;
use std::process::exit;
use std::time::Instant;
use zio_sendfile::copy_callback;

fn main() {
    let mut files = args().skip(1);
    let (first, second) = match (files.next(), files.next()) {
        (Some(first), Some(second)) => (first, second),
        _ => {
            eprintln!("requires two path arguments");
            exit(1);
        }
    };

    eprintln!("copying {} to {}", first, second);
    let mut first = File::open(first).expect("failed to open first file");
    let mut second = File::create(second).expect("failed to create second file");

    let len = first.metadata().ok().map_or(0, |m| m.len());

    let start = Instant::now();
    let result = copy_callback(
        // Copy 100 MiB per syscall
        100 * 1024 * 1024,
        &mut first,
        &mut second,
        |sf, wrote| {
            let offset = sf.offset as u64;

            let seconds = Instant::now().duration_since(start).as_secs();
            if seconds < 1 {
                return;
            }

            println!(
                "{}s: {} MiB of {} MiB ({}% {} MiB/s)",
                seconds,
                offset / 1024 / 1024,
                len / 1024 / 1024,
                offset * 100 / len,
                offset / seconds / 1024 / 1024
            );
        },
    );

    println!("result: {:?}", result);
}
