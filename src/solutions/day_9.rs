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

    part_one(&height_map); // 594
    part_two(&height_map); // 858494
}

fn part_one(height_map: &Vec<Vec<i32>>) {
    let mut total_risk_level = 0;
    for y in 0..height_map.len() {
        for x in 0..height_map[0].len() {
            if is_lowest(height_map, x, y) {
                total_risk_level += 1 + height_map[y][x];
            }
        }
    }

    println!("Part 1: {}", total_risk_level);
}

fn part_two(height_map: &Vec<Vec<i32>>) {
    let mut basins: Vec<i32> = vec!();
    for y in 0..height_map.len() {
        for x in 0..height_map[0].len() {
            if is_lowest(height_map, x, y) {
                basins.push(basin_size(height_map, x, y, vec!()).0);
            }
        }
    }

    basins.sort();
    let mut sizes_mult = 1;
    for i in 1..4 {
        sizes_mult *= basins[basins.len()-i];
    }
    println!("Part 2: {}", sizes_mult);
}

fn is_lowest(height_map: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let height = height_map[y][x];

    let mut lowest = x == 0 || height < height_map[y][x-1];
    lowest = lowest && (x == height_map[0].len() - 1 || height < height_map[y][x+1]);
    lowest = lowest && (y == 0 || height < height_map[y-1][x]);
    lowest = lowest && (y == height_map.len() - 1 || height < height_map[y+1][x]);

    lowest
}

fn basin_size(height_map: &Vec<Vec<i32>>, x: usize, y: usize, mut visited_coords: Vec<(usize, usize)>) -> (i32, Vec<(usize, usize)>) {
    let height = height_map[y][x];
    visited_coords.push((x, y));
    let mut total_size = 1;

    if x != 0 && !visited_coords.contains(&(x-1, y)) && height < height_map[y][x-1] && height_map[y][x-1] != 9 {
        let (size, coords) = basin_size(height_map, x-1, y, visited_coords);
        visited_coords = coords;
        total_size += size;
    }

    if x != height_map[0].len() - 1 && !visited_coords.contains(&(x+1, y)) && height < height_map[y][x+1] && height_map[y][x+1] != 9 {
        let (size, coords) = basin_size(height_map, x+1, y, visited_coords);
        visited_coords = coords;
        total_size += size;
    }

    if y != 0 && !visited_coords.contains(&(x, y-1)) && height < height_map[y-1][x] && height_map[y-1][x] != 9 {
        let (size, coords) = basin_size(height_map, x, y-1, visited_coords);
        visited_coords = coords;
        total_size += size;
    }

    if y != height_map.len() - 1 && !visited_coords.contains(&(x, y+1)) && height < height_map[y+1][x] && height_map[y+1][x] != 9 {
        let (size, coords) = basin_size(height_map, x, y+1, visited_coords);
        visited_coords = coords;
        total_size += size;
    }

    (total_size, visited_coords)
}
