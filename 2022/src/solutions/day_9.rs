use std::collections::HashSet;

pub fn run() {
    let input_str = include_str!("../../inputs/input_9.txt");

    let input: Vec<(&str, u32)> = input_str
        .lines()
        .map(|line| {
            let line_split: Vec<&str> = line
                .split_ascii_whitespace()
                .collect();

            (line_split[0], line_split[1].parse().unwrap())
        }).collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<(&str, u32)>) {
    let mut coords_visited: HashSet<(i32, i32)> = HashSet::new();
    coords_visited.insert((0, 0));

    let mut head_coords = (0, 0);
    let mut tail_coords = (0, 0);

    for &(direction, magnitude) in input {
        let x_change = match direction {
            "L" => -1,
            "R" => 1,
            _ => 0
        };
        let y_change = match direction {
            "D" => -1,
            "U" => 1,
            _ => 0
        };

        for _ in 0..magnitude {
            head_coords.0 += x_change;
            head_coords.1 += y_change;

            let x_diff: i32 = head_coords.0 - tail_coords.0;
            let y_diff: i32 = head_coords.1 - tail_coords.1;

            if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
                continue;
            }
            else if x_diff == 0 {
                tail_coords.1 += y_change;
            }
            else if y_diff == 0 {
                tail_coords.0 += x_change;
            }
            else {
                tail_coords.0 += if x_diff > 0 { 1 } else { -1 };
                tail_coords.1 += if y_diff > 0 { 1 } else { -1 };
            }

            coords_visited.insert(tail_coords);
        }
    }

    println!("Part one: {}", coords_visited.len());
}

fn part_two(input: &Vec<(&str, u32)>) {
    let mut coords_visited: HashSet<(i32, i32)> = HashSet::new();
    coords_visited.insert((0, 0));

    let mut sections: [(i32, i32); 10] = [(0, 0); 10];

    for &(direction, magnitude) in input {
        let x_change = match direction {
            "L" => -1,
            "R" => 1,
            _ => 0
        };
        let y_change = match direction {
            "D" => -1,
            "U" => 1,
            _ => 0
        };

        for _ in 0..magnitude {
            sections[0].0 += x_change;
            sections[0].1 += y_change;

            for i in 1..=9 {
                let x_diff: i32 = sections[i-1].0 - sections[i].0;
                let y_diff: i32 = sections[i-1].1 - sections[i].1;

                if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
                    continue;
                }
                else if x_diff == 0 {
                    sections[i].1 += if y_diff > 0 { 1 } else { -1 };
                }
                else if y_diff == 0 {
                    sections[i].0 += if x_diff > 0 { 1 } else { -1 };
                }
                else {
                    sections[i].0 += if x_diff > 0 { 1 } else { -1 };
                    sections[i].1 += if y_diff > 0 { 1 } else { -1 };
                }
            }

            coords_visited.insert(sections[9]);
        }
    }

    println!("Part two: {}", coords_visited.len());
}
