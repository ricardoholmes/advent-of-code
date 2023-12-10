use std::collections::{HashMap, HashSet};

use crate::safe_unpack;

type Coord = (i32, i32);

pub fn parse(input_raw: &str) -> Result<(Coord, HashMap<Coord, (Coord, Coord)>), String> {
    let lines = input_raw
        .lines();

    let mut map = HashMap::new();
    let mut pipes = HashMap::new();
    let mut start = (0, 0);

    for (y, line) in lines.clone().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), c);
            if c == 'S' {
                let x_scaled = 1 + 3 * x as i32;
                let y_scaled = 1 + 3 * y as i32;

                let mut directions: Vec<(i32, i32)> = vec![];
                if x > 0 && ['-', 'F', 'L'].contains(&line.chars().nth(x - 1).unwrap_or('.')) {
                    directions.push((-1, 0));
                }
                if ['-', '7', 'J'].contains(&line.chars().nth(x + 1).unwrap_or('.')) {
                    directions.push((1, 0));
                }
                if y > 0 && ['|', 'F', '7'].contains(&lines.clone().nth(y - 1).unwrap_or("").chars().nth(x).unwrap_or('.')) {
                    directions.push((0, -1));
                }
                if ['|', 'L', 'J'].contains(&lines.clone().nth(y + 1).unwrap_or("").chars().nth(x).unwrap_or('.')) {
                    directions.push((0, 1));
                }

                if directions.len() != 2 {
                    panic!();
                }

                safe_unpack!(directions.iter(), first, second);

                start = (x_scaled, y_scaled);
                pipes.insert((x_scaled, y_scaled), ((x_scaled + first.0, y_scaled + first.1), (x_scaled + second.0, y_scaled + second.1)));

                if directions.contains(&(-1, 0)) {
                    pipes.insert((x_scaled - 1, y_scaled), ((x_scaled - 2, y_scaled), (x_scaled, y_scaled)));
                }
                if directions.contains(&(1, 0)) {
                    pipes.insert((x_scaled + 1, y_scaled), ((x_scaled + 2, y_scaled), (x_scaled, y_scaled)));
                }
                if directions.contains(&(0, -1)) {
                    pipes.insert((x_scaled, y_scaled - 1), ((x_scaled, y_scaled - 2), (x_scaled, y_scaled)));
                }
                if directions.contains(&(0, 1)) {
                    pipes.insert((x_scaled, y_scaled + 1), ((x_scaled, y_scaled + 2), (x_scaled, y_scaled)));
                }
            }
            else if c != '.' {
                let x = 1 + 3 * x as i32;
                let y = 1 + 3 * y as i32;
                match c {
                    '-' => {
                        for x in x-1..=x+1 {
                            pipes.insert((x, y), ((x - 1, y), (x + 1, y)));
                        }
                    },
                    '|' => {
                        for y in y-1..=y+1 {
                            pipes.insert((x, y), ((x, y - 1), (x, y + 1)));
                        }
                    },
                    'F' => {
                        pipes.insert((x, y), ((x, y + 1), (x + 1, y)));
                        pipes.insert((x, y + 1), ((x, y), (x, y + 2)));
                        pipes.insert((x + 1, y), ((x, y), (x + 2, y)));
                    },
                    '7' => {
                        pipes.insert((x, y), ((x, y + 1), (x - 1, y)));
                        pipes.insert((x, y + 1), ((x, y), (x, y + 2)));
                        pipes.insert((x - 1, y), ((x, y), (x - 2, y)));
                    },
                    'L' => {
                        pipes.insert((x, y), ((x, y - 1), (x + 1, y)));
                        pipes.insert((x, y - 1), ((x, y), (x, y - 2)));
                        pipes.insert((x + 1, y), ((x, y), (x + 2, y)));
                    },
                    'J' => {
                        pipes.insert((x, y), ((x - 1, y), (x, y - 1)));
                        pipes.insert((x - 1, y), ((x, y), (x - 2, y)));
                        pipes.insert((x, y - 1), ((x, y), (x, y - 2)));
                    },
                    _ => panic!(),
                }
                pipes.insert((x, y), match c {
                    '-' => ((x.max(1) - 1, y), (x + 1, y)),
                    '|' => ((x, y.max(1) - 1), (x, y + 1)),
                    'F' => ((x, y + 1), (x + 1, y)),
                    '7' => ((x, y + 1), (x.max(1) - 1, y)),
                    'L' => ((x + 1, y), (x, y.max(1) - 1)),
                    'J' => ((x.max(1) - 1, y), (x, y.max(1) - 1)),
                    _ => panic!(),
                });
            }
        }
    }

    Ok((start, pipes))
}

pub fn part_one(input: &(Coord, HashMap<Coord, (Coord, Coord)>)) -> Result<u32, String> {
    let (start, pipes) = input;

    let mut loop_length = 0;
    let mut prev_node = *start;
    let mut node = *start;
    loop {
        let connections = pipes.get(&node).unwrap();
        if prev_node == connections.0 {
            prev_node = node;
            node = connections.1;
        }
        else {
            prev_node = node;
            node = connections.0;
        }
        loop_length += 1;

        if node == *start {
            break;
        }
    }

    Ok((loop_length / 3) / 2)
}

pub fn part_two(input: &(Coord, HashMap<Coord, (Coord, Coord)>)) -> Result<usize, String> {
    let (start, pipes) = input;

    let mut loop_coords = HashSet::new();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    let mut prev_node = *start;
    let mut node = *start;

    loop {
        let connections = pipes.get(&node).unwrap();
        if prev_node == connections.0 {
            prev_node = node;
            node = connections.1;
        }
        else {
            prev_node = node;
            node = connections.0;
        }
        loop_coords.insert(node);

        if node.0 > max_x {
            max_x = node.0;
        }
        if node.0 < min_x {
            min_x = node.0;
        }
        if node.1 > max_y {
            max_y = node.1;
        }
        if node.1 < min_y {
            min_y = node.1;
        }

        if node == *start {
            break;
        }
    }

    let mut enclosed = 0;
    let mut checked: HashSet<Coord> = HashSet::new();
    let mut enclosures: HashSet<Coord> = HashSet::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if checked.contains(&(x, y)) || loop_coords.contains(&(x, y)) {
                continue;
            }

            let mut failed = false;
            let mut nodes_to_check = vec![(x, y)];
            let mut potential_enclosure = HashSet::new();
            while !failed && nodes_to_check.len() > 0 {
                let node = nodes_to_check.pop().unwrap();
                if loop_coords.contains(&node) || potential_enclosure.contains(&node) {
                    continue;
                }

                if node.0 == 0 || node.1 == 0 || node.0 < min_x || node.0 > max_x || node.1 < min_y || node.1 > max_y {
                    failed = true;
                    break;
                }
                else if checked.contains(&node) {
                    // since this wouldn't have been entered if it was a valid enclosure,
                    // must have exited early due to a fail state
                    failed = true;
                    break;
                }

                nodes_to_check.push((node.0 - 1, node.1));
                nodes_to_check.push((node.0 + 1, node.1));
                nodes_to_check.push((node.0, node.1 - 1));
                nodes_to_check.push((node.0, node.1 + 1));
                potential_enclosure.insert(node);
            }

            if !failed {
                for i in &potential_enclosure {
                    let mut valid = true;
                    'outer: for offset_x in -1..=1 {
                        for offset_y in -1..=1 {
                            if !potential_enclosure.contains(&(i.0 + offset_x, i.1 + offset_y)) {
                                valid = false;
                                break 'outer;
                            }
                        }
                    }
                    if valid {
                        enclosed += 1;
                    }
                }
                enclosures.extend(potential_enclosure.clone());
            }
            checked.extend(potential_enclosure);
        }
    }

    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         if loop_coords.contains(&(x, y)) {
    //             print!("-");
    //         }
    //         else if enclosures.contains(&(x, y)) {
    //             print!("I");
    //         }
    //         else {
    //             print!("O");
    //         }
    //     }
    //     println!();
    // }

    Ok(enclosed / 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let example = include_str!("../../examples/day_10_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(4));
    }

    #[test]
    fn test_part1_2() {
        let example = include_str!("../../examples/day_10_2.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(8));
    }

    #[test]
    fn test_part2_1() {
        let example: &str = include_str!("../../examples/day_10_3.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(4));
    }

    #[test]
    fn test_part2_2() {
        let example: &str = include_str!("../../examples/day_10_4.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(8));
    }

    #[test]
    fn test_part2_3() {
        let example: &str = include_str!("../../examples/day_10_5.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(10));
    }
}
