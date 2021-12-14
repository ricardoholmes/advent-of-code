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

    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    for line in &input[0] {
        let line_split: Vec<&str> = line
            .split(",")
            .collect();

        grid.insert((line_split[0].parse().unwrap(), line_split[1].parse().unwrap()));
    }

    let mut folds: Vec<(&str, i32)> = vec!();
    for line in &input[1] {
        let line_split: Vec<&str> = line
            .split("=")
            .collect();

        folds.push((line_split[0], line_split[1].parse().unwrap()));
    }

    part_one(&grid, &folds);
    part_two(grid, &folds);
}

fn part_one(grid: &HashSet<(i32, i32)>, folds: &Vec<(&str, i32)>) {
    println!("Part 1: {}", fold_grid(grid, folds[0]).len());
}

fn part_two(mut grid: HashSet<(i32, i32)>, folds: &Vec<(&str, i32)>) {
    for fold in folds {
        grid = fold_grid(&grid, *fold);
    }

    let mut grid_width = 0;
    let mut grid_height = 0;
    for i in &grid {
        if i.0 + 1 > grid_width {
            grid_width = i.0 + 1;
        }

        if i.1 + 1> grid_height {
            grid_height = i.1 + 1;
        }
    }

    println!("Part 2:");
    for i in 0..grid_height {
        for j in 0..grid_width {
            if grid.contains(&(j, i)) {
                print!("#");
            }
            else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn fold_grid(grid: &HashSet<(i32, i32)>, fold: (&str, i32)) -> HashSet<(i32, i32)> {
    let mut cells: HashSet<(i32, i32)> = HashSet::new();
    if fold.0 == "x" {
        for cell in grid {
            if cell.0 > fold.1 {
                cells.insert((fold.1 - (cell.0 - fold.1), cell.1));
            }
            else {
                cells.insert((cell.0, cell.1));
            }
        }
    }

    else {
        for cell in grid {
            if cell.1 > fold.1 {
                cells.insert((cell.0, fold.1 - (cell.1 - fold.1)));
            }
            else {
                cells.insert((cell.0, cell.1));
            }
        }
    }

    cells
}
