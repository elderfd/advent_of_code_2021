use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input(file_name: &String) -> Vec<String> {
    let file = File::open(file_name);

    if file.is_err() {
        panic!("Could not open {}", file_name);
    }

    let mut input: Vec<String> = Vec::new();

    for line in io::BufReader::new(file.unwrap()).lines() {
        match line {
            Ok(v) => input.push(v),
            Err(_) => panic!("Failed to read line"),
        }
    }

    return input;
}
