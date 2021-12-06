use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input_6.txt")
        .expect("Failed to read file");

    let input: Vec<i32> = input
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();

    part_one(&input);
}

fn part_one(input: &Vec<i32>) {
    let mut fish_spawns: [i32; 8] = [0; 8];

    for i in 0..8 {
        let mut fish: Vec<i32> = vec!(i);
        for _j in 0..80 {
            let mut fish_temp: Vec<i32> = vec!();
            for k in fish {
                if k == 0 {
                    fish_temp.push(6);
                    fish_temp.push(8);
                }
                else {
                    fish_temp.push(k-1);
                }
            }
            fish = fish_temp;
        }
        fish_spawns[i as usize] = fish.len() as i32;
    }

    let mut count = 0;
    for i in input {
        count += fish_spawns[*i as usize];
    }

    println!("Part 1: {}", count);
}
