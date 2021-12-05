use std::fs;
use std::collections::HashMap;

pub fn run() {
    let input = fs::read_to_string("inputs/input_5.txt")
        .expect("Failed to read file");

    let input: Vec<&str> = input
        .split("\n")
        .collect();

    let mut lines: Vec<[[i32; 2]; 2]> = vec!();
    for line in input {
        let line_split: Vec<&str> = line
            .split(" -> ")
            .collect();

        let start_point: Vec<i32> = line_split[0]
            .split(",")
            .map(|s| s.parse().expect("parse error"))
            .collect();

        let end_point: Vec<i32> = line_split[1]
            .split(",")
            .map(|s| s.parse().expect("parse error"))
            .collect();

        lines.push([[start_point[0], start_point[1]], [end_point[0], end_point[1]]]);
    }

    part_one(&lines);
}

fn part_one(lines: &Vec<[[i32; 2]; 2]>) {
    let mut count = 0;
    let mut points_crossed: HashMap<[i32; 2], i32> = HashMap::new();
    for line in lines {
        // if the line is vertical
        if line[0][0] == line[1][0] {
            let start;
            let end;

            if line[0][1] < line[1][1] {
                start = line[0][1];
                end = line[1][1];
            }

            else {
                start = line[1][1];
                end = line[0][1];
            }

            for i in start..end+1 {
                let point = [line[0][0], i];
                if points_crossed.contains_key(&point) {
                    if points_crossed[&point] == 1 {
                        count += 1
                    }

                    *points_crossed.get_mut(&point).unwrap() += 1;
                }

                else {
                    points_crossed.insert(point, 1);
                }
            }
        }

        // if the line is vertical
        else if line[0][1] == line[1][1] {
            let start;
            let end;

            if line[0][0] < line[1][0] {
                start = line[0][0];
                end = line[1][0];
            }

            else {
                start = line[1][0];
                end = line[0][0];
            }

            for i in start..end+1 {
                let point = [i, line[0][1]];
                if points_crossed.contains_key(&point) {
                    if points_crossed[&point] == 1 {
                        count += 1
                    }

                    *points_crossed.get_mut(&point).unwrap() += 1;
                }

                else {
                    points_crossed.insert(point, 1);
                }
            }
        }
    }

    println!("Part 1: {}", count);
}
