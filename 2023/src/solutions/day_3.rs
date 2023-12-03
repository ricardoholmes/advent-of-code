use colored::Colorize;
use std::collections::HashSet;

pub fn run() -> Result<(), String> {
    let input: Vec<Vec<char>> = include_str!("../../inputs/input_3.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    match part_one(&input) {
        Ok(n) => println!("Part one: {n}"),
        Err(e) => return Err(e),
    };

    match part_two(&input) {
        Ok(n) => println!("Part two: {n}"),
        Err(e) => return Err(e),
    };

    Ok(())
}

fn part_one(input: &Vec<Vec<char>>) -> Result<u32, String> {
    let mut coords: HashSet<(usize, usize)> = HashSet::new();
    let mut adjacent_coords: HashSet<(usize, usize)> = HashSet::new();

    let mut total: u32 = 0;
    for line_index in 0..input.len() {
        let line = input.get(line_index).unwrap();

        let mut maybe_coord: HashSet<(usize, usize)> = HashSet::new();
        let mut num_str = String::new();
        let mut symbol_adjacent = false;
        for char_index in 0..line.len() {
            let c = line.get(char_index).unwrap().to_owned();

            if !c.is_numeric() && num_str.len() > 0 {
                if symbol_adjacent {
                    coords.extend(&maybe_coord);
                    total += num_str.parse::<u32>().unwrap();
                }
                num_str.clear();
                maybe_coord.clear();
                symbol_adjacent = false;
            }
            else if c.is_numeric() {
                maybe_coord.insert((line_index, char_index));
                num_str.push(c);
                if !symbol_adjacent {
                    'outer: for y in (line_index.max(1)-1)..=(line_index+1).min(input.len()-1) {
                        let adj_line = input.get(y).unwrap();
                        for x in (char_index.max(1)-1)..=(char_index+1).min(adj_line.len()-1) {
                            let adj_char = adj_line.get(x).unwrap().to_owned();
                            if !adj_char.is_numeric() && adj_char != '.' {
                                adjacent_coords.insert((y, x));
                                symbol_adjacent = true;
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        if num_str.len() > 0 && symbol_adjacent {
            coords.extend(&maybe_coord);
            total += num_str.parse::<u32>().unwrap();
        }
    }

    Ok(total)
}

fn part_two(input: &Vec<Vec<char>>) -> Result<u32, String> {
    Ok(0)
}

fn visualise(input: &Vec<Vec<char>>, num_coords: &HashSet<(usize, usize)>, adj_coords: &HashSet<(usize, usize)>) {
    for line_index in 0..input.len() {
        let line = input.get(line_index).unwrap();
        for char_index in 0..line.len() {
            let c = line.get(char_index).unwrap().to_owned();
            if num_coords.contains(&(line_index, char_index)) {
                print!("{}", c.to_string().cyan());
            }
            else if c.is_numeric() {
                print!("{}", c.to_string().red());
            }
            else if adj_coords.contains(&(line_index, char_index)) {
                print!("{}", c.to_string().yellow());
            }
            else {
                print!("{c}");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example: Vec<Vec<char>> = include_str!("../../examples/day3_1.txt")
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let result = part_one(&example);
        assert_eq!(result, Ok(4361));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, 1);
    }
}
