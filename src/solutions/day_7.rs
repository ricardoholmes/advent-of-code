use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input_7.txt")
        .expect("Failed to read file");

    let input: Vec<i32> = input
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();
    
    part_one(&input);
}

fn part_one(positions: &Vec<i32>) {
    let mut new_position = 0;
    let mut previous_fuel = -1;
    loop {
        let mut fuel = 0;
        for i in positions {
            fuel += (i - new_position).abs();
        }

        if previous_fuel >= 0 && fuel > previous_fuel {
            println!("Part 1: {}", previous_fuel);
            return ();
        }
        previous_fuel = fuel;
        new_position += 1;
    }
}
