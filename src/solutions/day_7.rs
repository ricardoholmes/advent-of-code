use std::collections::HashMap;

pub fn run() {
    let input: Vec<&str> = include_str!("../../inputs/input_7.txt")
        .lines()
        .collect();

    /*
    let commands: Vec<&str> = input
        .iter()
        .filter(|line| (*line).starts_with('$'))
        .collect();
    */

    let sizes = part_one(&input);
    part_two(&sizes);
}

fn part_one(input: &[&str]) -> HashMap<String, i32> {
    let mut directory_sizes: HashMap<String, i32> = HashMap::new();
    let mut path: Vec<String> = vec!();

    let mut size = 0;

    for line in input {
        if line.starts_with('$') {
            if !path.is_empty() && (size != 0 || !directory_sizes.contains_key(&path.join("/"))) {
                directory_sizes.insert(path.join("/"), size);
            }

            if line.starts_with("$ cd") {
                let dir = line.chars().skip(5).collect::<String>();
                size = 0;
                if dir == ".." {
                    path.pop();
                }
                else {
                    path.push(dir);
                }
            }
        }
        else if line.chars().nth(0).unwrap().is_digit(10) {
            size += line.split(' ').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        }
    }

    directory_sizes.insert(path.join("/"), size);

    directory_sizes = directory_sizes
        .iter()
        .map(|(path, _)| (path.to_owned(), directory_sizes
            .iter()
            .fold(0, |sum, (p, s)| sum + if p.starts_with(path) { *s } else { 0 })
        )).collect();

    let mut total = 0;
    for dir in &directory_sizes {
        if *dir.1 <= 100_000 {
            total += dir.1;
        }
    }

    println!("Part one: {total}");

    directory_sizes
}

fn part_two(sizes: &HashMap<String, i32>) {
    // max used space = 70_000_000 - 30_000_000 = 40_000_000
    let space_needed = *sizes.get("/").unwrap() - 40_000_000;

    let size = sizes
        .values()
        .filter(|size| **size >= space_needed)
        .min()
        .unwrap();

    println!("Part two: {size}");
}
