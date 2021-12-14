use std::fs;
use std::collections::HashMap;

pub fn run() {
    let input_str = fs::read_to_string("inputs/input_14.txt")
        .expect("Failed to read file");

    let input: Vec<&str> = input_str
        .split("\n\n")
        .collect();

    part_one(&input)
}

fn part_one(input: &Vec<&str>) {
    let mut pair_insertion_rules: HashMap<&str, &str> = HashMap::new();
    for line in input[1].split("\n") {
        let line_split: Vec<&str> = line
            .split(" -> ")
            .collect();

        pair_insertion_rules.insert(line_split[0], line_split[1]);
    }

    let mut polymer: String = input[0].to_string();
    for _step in 0..10 {
        let mut polymer_temp: String = String::new();
        for i in 0..polymer.len()-1 {
            polymer_temp += &polymer.chars().nth(i).unwrap().to_string();
            let pair = &polymer[i..i+2];
            polymer_temp += pair_insertion_rules.get(pair).unwrap();
        }
        polymer_temp += &polymer.chars().nth(polymer.len()-1).unwrap().to_string();
        polymer = polymer_temp;
    }


    let mut min_count = 0;

    let mut max_count = 0;

    for i in polymer.chars() {
        let count = polymer.matches(i).count();
        if count > max_count {
            max_count = count;
        }

        if min_count == 0 || count < min_count {
            min_count = count;
        }
    }

    println!("Part 1: {}", max_count - min_count);
}
