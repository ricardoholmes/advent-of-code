use std::collections::HashSet;

pub fn run() {
    let input_str = include_str!("../../inputs/input_18.txt");

    let input: Vec<Vec<i32>> = input_str
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[Vec<i32>]) {
    let mut count = 0;

    for cube in input {
        for i in 0..3 {
            for j in &[-1, 1] {
                let mut new_cube = cube.clone();
                new_cube[i] += j;
                if !input.contains(&new_cube) {
                    count += 1;
                }
            }
        }
    }

    println!("Part one: {count}");
}

fn part_two(input: &[Vec<i32>]) {
    let mut count = 0;

    for cube in input {
        for i in 0..3 {
            for j in &[-1, 1] {
                let mut adjacent_cube = cube.clone();
                adjacent_cube[i] += j;
                if !input.contains(&adjacent_cube) && !is_encased(&adjacent_cube, input) {
                    count += 1;
                }
            }
        }
    }

    println!("Part two: {count}");
}

fn is_encased(cube: &[i32], map: &[Vec<i32>]) -> bool {
    let mut cubes: Vec<Vec<i32>> = vec![];
    cubes.push(cube.to_vec());

    let mut checked_cubes: HashSet<Vec<i32>> = HashSet::new();

    while !cubes.is_empty() {
        let cube = cubes.pop().unwrap();
        if !raycast(&cube, map) {
            return false;
        }
        checked_cubes.insert(cube.clone());

        for i in 0..3 {
            for j in &[-1, 1] {
                let mut adjacent_cube = cube.clone();
                adjacent_cube[i] += j;
                if !checked_cubes.contains(&adjacent_cube) && !map.contains(&adjacent_cube) && !cubes.contains(&adjacent_cube) {
                    cubes.push(adjacent_cube);
                }
            }
        }
    }

    true
}

fn raycast(cube: &[i32], map: &[Vec<i32>]) -> bool {
    for i in 0..3 {
        if map.iter().filter(|lava|
            lava[i] > cube[i] && lava[(i+1) % 3] == cube[(i+1) % 3] && lava[(i+2) % 3] == cube[(i+2) % 3]
        ).count() == 0 {
            return false;
        }
        if map.iter().filter(|lava|
            lava[i] < cube[i] && lava[(i+1) % 3] == cube[(i+1) % 3] && lava[(i+2) % 3] == cube[(i+2) % 3]
        ).count() == 0 {
            return  false;
        }
    }

    true
}
