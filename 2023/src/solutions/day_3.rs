use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Num {
    value: u32,
    start_pos: (usize, usize),
}

#[derive(Clone)]
struct Symbol {
    ch: char,
    nums_adjacent: Vec<Num>,
}

pub fn run(input_raw: &str) -> Result<(), String> {
    let symbols = parse(input_raw)?;

    let answer_part_one = part_one(&symbols)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&symbols)?;
    println!("Part two: {}", answer_part_two);

    Ok(())
}

fn parse(input_raw: &str) -> Result<Vec<Symbol>, String> {
    let input: Vec<Vec<char>> = input_raw
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut symbols: Vec<Symbol> = vec![];

    let mut num_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut symbol_coords: HashSet<(usize, usize)> = HashSet::new();

    for line_index in 0..input.len() {
        let line = input.get(line_index).unwrap();
        for char_index in 0..line.len() {
            let ch = line.get(char_index).unwrap().to_owned();

            if ch.is_numeric() || ch == '.' {
                continue;
            }

            let mut nums: Vec<Num> = vec![];
            let mut num_coords_adjacent: HashSet<(usize, usize)> = HashSet::new();

            let lines_range = (line_index.max(1)-1)..=(line_index+1).min(input.len()-1);
            for y in lines_range {
                let adj_line = input.get(y).unwrap();
                let chars_range = (char_index.max(1)-1)..=(char_index+1).min(adj_line.len()-1);
                for x in chars_range {
                    if num_coords_adjacent.contains(&(y, x)) {
                        continue;
                    }

                    let adj_char = adj_line.get(x).unwrap().to_owned();
                    if adj_char.is_numeric() && adj_char != '.' {
                        let num = get_num(&mut num_coords_adjacent, adj_line, x, y);
                        nums.push(num);
                    }
                }
            }

            if nums.len() > 0 {
                symbols.push(Symbol {
                    ch,
                    nums_adjacent: nums
                });

                num_coords.extend(&num_coords_adjacent);
                symbol_coords.insert((line_index, char_index));
            }
        }
    }

    Ok(symbols)
}

fn get_num(num_coords: &mut HashSet<(usize, usize)>, line: &Vec<char>, x: usize, y: usize) -> Num {
    let mut start_x = 0;
    let mut num_str = String::new();
    for char_index in (0..x).rev() {
        let ch = line.get(char_index).unwrap().to_owned();
        if ch.is_numeric() {
            num_str.push(ch);
            num_coords.insert((y, char_index));
        } else {
            start_x = char_index + 1;
            break;
        }
    }
    num_str = num_str.chars().rev().collect();

    for char_index in x..line.len() {
        let ch = line.get(char_index).unwrap().to_owned();
        if ch.is_numeric() {
            num_str.push(ch);
            num_coords.insert((y, char_index));
        } else {
            break;
        }
    }

    Num {
        value: num_str.parse().unwrap(),
        start_pos: (y, start_x),
    }
}

fn part_one(symbols: &[Symbol]) -> Result<u32, String> {
    let mut total = 0;
    let mut start_coords: HashSet<(usize, usize)> = HashSet::new();
    for symbol in symbols {
        for num in symbol.nums_adjacent.clone() {
            if !start_coords.contains(&num.start_pos) {
                start_coords.insert(num.start_pos);
                total += num.value;
            }
        }
    }

    Ok(total)
}

fn part_two(symbols: &[Symbol]) -> Result<u32, String> {
    let mut total = 0;
    for symbol in symbols {
        let nums = symbol.nums_adjacent.clone();
        if symbol.ch != '*' || nums.len() != 2 {
            continue;
        }
        total += nums[0].value * nums[1].value;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_3_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_one(&parsed);
        assert_eq!(result, Ok(4361));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_3_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_two(&parsed);
        assert_eq!(result, Ok(467835));
    }
}
