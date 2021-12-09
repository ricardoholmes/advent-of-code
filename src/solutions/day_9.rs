use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input_9.txt")
        .expect("Failed to read file");

    let input: Vec<&str> = input
        .split("\n")
        .collect();

    let mut height_map: Vec<Vec<i32>> = vec!();
    for line in input {
        height_map.push(
            line
            .chars()
            .map(|s| s.to_string().parse().expect("parse error"))
            .collect()
        );
    }

    part_one(&height_map);
}

fn part_one(height_map: &Vec<Vec<i32>>) {
    let mut total_risk_level = 0;
    for y in 0..height_map.len() {
        for x in 0..height_map[0].len() {
            let height = height_map[y][x];

            let mut is_lowest = x == 0 || height < height_map[y][x-1];
            is_lowest = is_lowest && (x == height_map[0].len() - 1 || height < height_map[y][x+1]);
            is_lowest = is_lowest && (y == 0 || height < height_map[y-1][x]);
            is_lowest = is_lowest && (y == height_map.len() - 1 || height < height_map[y+1][x]);

            if is_lowest {
                total_risk_level += 1 + height;
            }
        }
    }

    println!("Part 1: {}", total_risk_level);
}
