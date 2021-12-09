use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input_8.txt")
        .expect("Failed to read file");

    // if input is a list of strings
    let input: Vec<&str> = input
        .split("\n")
        .collect();
    
    part_one(&input);
}

fn part_one(input: &Vec<&str>) {
    let mut count = 0;
    for line in input {
        let output: Vec<&str> = line
            .split(" | ")
            .collect();

        let output_split: Vec<&str> = output[1]
            .split(" ")
            .collect();

        for i in output_split {
            let length = i.chars().count();
            if length == 2 || length == 3 || length == 4 || length == 7 {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}
