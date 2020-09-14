use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = args()
        .nth(1)
        .expect("You must enter a filename to read line by line");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // Handle any errors that may arise
        match line {
            Ok(ln) => print!("{}", ln),
            Err(error) => print!("{}", error),
        }
    }
    println!();
}
