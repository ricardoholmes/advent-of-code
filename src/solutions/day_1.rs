use std::fs;

pub fn run() {
    let input_str = fs::read_to_string("inputs/input_1.txt")
        .expect("Failed to read file");

    let input: Vec<i32> = input_str
        .split("\n")
        .map(|s| s.parse().expect("parse error"))
        .collect();
    
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<i32>) {
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

fn part2(input: &Vec<i32>) {
    let mut previous = input[0] + input[1] + input[2];
    let mut increasing_count = 0;

    for i in 1..(input.len()-2) {
        let sum = input[i] + input[i+1] + input[i+2];
        if sum > previous {
            increasing_count += 1;
        }

        previous = sum;
    }
    println!("Part 2: {}", increasing_count);
}
