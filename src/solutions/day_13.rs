use std::fs;
use std::collections::HashSet;

pub fn run() {
    let input = fs::read_to_string("inputs/input_13.txt")
        .expect("Failed to read file")
        .replace("fold along ", "");

    let input: Vec<Vec<&str>>  = input
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>())
        .collect();

    let grid: Vec<Vec<i32>> = input[0]
        .iter()
        .map(|s| s.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>())
        .collect();

    let mut folds: Vec<(&str, i32)> = vec!();
    for line in &input[1] {
        let line_split: Vec<&str> = line
            .split("=")
            .collect();

        folds.push((line_split[0], line_split[1].parse().unwrap()));
    }

    part_one(&grid, folds[0]);
}

fn part_one(grid: &Vec<Vec<i32>>, fold: (&str, i32)) {
    let mut cells: HashSet<(i32, i32)> = HashSet::new();
    if fold.0 == "x" {
        for cell in grid {
            if cell[0] > fold.1 {
                cells.insert((fold.1 - (cell[0] - fold.1), cell[1]));
            }
            else {
                cells.insert((cell[0], cell[1]));
            }
        }
    }

    else {
        for cell in grid {
            if cell[1] > fold.1 {
                cells.insert((cell[0], fold.1 - (cell[1] - fold.1)));
            }
            else {
                cells.insert((cell[0], cell[1]));
            }
        }
    }

    println!("Part 1: {}", cells.len());
}
