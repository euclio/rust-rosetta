use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut args = args();

    let filename = args
        .nth(1)
        .expect("You must enter a filename to read line by line");
    let line_number = args
        .next()
        .and_then(|num| num.parse::<usize>().ok())
        .expect("You must enter an integer as the line number");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    match reader.lines().nth(line_number - 1) {
        None => panic!("No such line (file is too short)"),
        Some(result) => {
            match result {
                // Handle any errors that may arise
                Ok(ln) => print!("{}", ln),
                Err(error) => print!("{}", error),
            }
        }
    }
}
