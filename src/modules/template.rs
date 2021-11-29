use std::fs;

pub fn run() {
    let input_str = fs::read_to_string("inputs/input.txt")
        .expect("Failed to read file");

    // if input is a list of strings
    let input = input_str
        .split(",")
        .collect();

    // if input is a list of integers
    let input = input_str
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();
}
