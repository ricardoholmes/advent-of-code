use std::fs;

pub fn run() {
    let input_str = fs::read_to_string("inputs/input_1.txt")
        .expect("Failed to read file");

    let input: Vec<i32> = input_str
        .split("\n")
        .map(|s| s.parse().expect("parse error"))
        .collect();
    
    let mut previous = input[0];
    let mut increasing_count = 0;
    for i in &input[1..] {
        if *i > previous {
            increasing_count += 1;
        }

        previous = *i;
    }

    println!("Part 1: {}", increasing_count);
}
