use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input_6.txt")
        .expect("Failed to read file");

    let input: Vec<i32> = input
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<i32>) {
    let count = amount_after_days(&input, 80);
    println!("Part 1: {}", count);
}

fn part_two(input: &Vec<i32>) {
    let count = amount_after_days(&input, 256);
    println!("Part 2: {}", count);
}

fn amount_after_days(input: &Vec<i32>, days: i32) -> u64 {
    let mut fish: [u64; 9] = [0; 9];

    for i in input {
        fish[*i as usize] += 1;
    }

    for _i in 0..days {
        let mut fish_temp: [u64; 9] = [0; 9];
        fish_temp[8] += fish[0];
        fish_temp[6] += fish[0];
        for j in 1..9 {
            fish_temp[j-1] += fish[j];
        }
        fish = fish_temp;
    }

    fish.iter().sum()
}
