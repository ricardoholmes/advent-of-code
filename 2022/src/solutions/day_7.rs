use std::collections::HashMap;

pub fn run() {
    let input: Vec<&str> = include_str!("../../inputs/input_7.txt")
        .lines()
        .collect();

    let sizes = part_one(&input);
    part_two(&sizes);
}

fn part_one(input: &[&str]) -> HashMap<String, u32> {
    let mut directory_sizes: HashMap<String, u32> = HashMap::new();
    let mut path: Vec<String> = vec!();

    let mut size = 0;

    for line in input {
        // if line is a command
        if line.starts_with('$') {
            // ensure directory_sizes contains every directory thats entered
            if !path.is_empty() && (size != 0 || !directory_sizes.contains_key(&path.join("/"))) {
                directory_sizes.insert(path.join("/"), size);
            }

            // update current path
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

        else if line.chars().next().unwrap_or('\0').is_ascii_digit() {
            size += line
                .split_whitespace()
                .collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap_or(0);
        }
    }

    // in case it ends with ls output
    directory_sizes.insert(path.join("/"), size);

    // calculate size of upper directories
    directory_sizes = directory_sizes
        .iter()
        .map(|(path, _)| (path.to_owned(), directory_sizes
            .iter()
            .fold(0, |sum, (dir, size)| sum + if dir.starts_with(path) { *size } else { 0 })
        )).collect();

    // calculate sum of sizes of all directories less than or equal to 100_000 units in size
    let total: u32 = directory_sizes
        .values()
        .filter(|size| **size <= 100_000)
        .sum();

    println!("Part one: {total}");

    directory_sizes
}

fn part_two(sizes: &HashMap<String, u32>) {
    // max used space = 70_000_000 - 30_000_000 = 40_000_000
    // calculate space needed by using max used space and total space used (space used by root directory)
    let space_needed: u32 = *sizes.get("/").unwrap_or(&40_000_000) - 40_000_000;

    // find smallest value in the hashmap of directory sizes that is bigger than the space needed
    let size: u32 = *sizes
        .values()
        .filter(|size| **size >= space_needed)
        .min()
        .unwrap_or(&0);

    println!("Part two: {size}");
}
