use std::collections::HashSet;

type Parsed = (Direction, u32, String);

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut parsed = vec![];

    for line in input_raw.lines() {
        let mut line_split = line.split_ascii_whitespace();

        let direction = match line_split.next() {
            Some("L") => Direction::Left,
            Some("R") => Direction::Right,
            Some("U") => Direction::Up,
            Some("D") => Direction::Down,
            _ => return Err(format!("Invalid input string")),
        };

        let amount = line_split.next().unwrap().parse().unwrap();

        let mut color = line_split.next().unwrap();
        color = &color[1..(color.len()-2)];

        parsed.push((direction, amount, color.to_string()));
    }

    Ok(parsed)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut trench_positions = HashSet::new();
    trench_positions.insert((0, 0));

    let mut x_bounds = (0, 0);
    let mut y_bounds = (0, 0);
    let mut position = (0, 0);
    for i in input {
        for _ in 0..i.1 {
            position = match i.0 {
                Direction::Left => (position.0 - 1, position.1),
                Direction::Right => (position.0 + 1, position.1),
                Direction::Up => (position.0, position.1 - 1),
                Direction::Down => (position.0, position.1 + 1),
            };

            if position.0 < x_bounds.0 {
                x_bounds.0 = position.0;
            }
            if position.0 > x_bounds.1 {
                x_bounds.1 = position.0;
            }
            
            if position.1 < y_bounds.0 {
                y_bounds.0 = position.1;
            }
            if position.1 > y_bounds.1 {
                y_bounds.1 = position.1;
            }

            trench_positions.insert(position);
        }
    }

    let mut out_of_bounds = HashSet::new();
    for y in y_bounds.0..=y_bounds.1 {
        for x in x_bounds.0..=x_bounds.1 {
            if out_of_bounds.contains(&(x, y)) || trench_positions.contains(&(x, y)) {
                continue;
            }

            let mut flood = HashSet::new();
            let mut flood_steps = vec![(x, y)];
            let mut is_in_trench = true;
            while !flood_steps.is_empty() {
                let pos = flood_steps.pop().unwrap();
                if pos.0 < x_bounds.0 || pos.0 > x_bounds.1 {
                    is_in_trench = false;
                    break;
                }
                if pos.1 < y_bounds.0 || pos.1 > y_bounds.1 {
                    is_in_trench = false;
                    break;
                }

                if flood.contains(&pos) || trench_positions.contains(&pos) {
                    continue;
                }

                flood.insert(pos);
                flood_steps.push((pos.0 - 1, pos.1));
                flood_steps.push((pos.0 + 1, pos.1));
                flood_steps.push((pos.0, pos.1 - 1));
                flood_steps.push((pos.0, pos.1 + 1));
            }

            if is_in_trench {
                trench_positions.extend(flood);
            }
            else {
                out_of_bounds.extend(flood);
            }
        }
    }

    // for y in y_bounds.0..=y_bounds.1 {
    //     for x in x_bounds.0..=x_bounds.1 {
    //         if trench_positions.contains(&(x, y)) {
    //             print!("#");
    //         }
    //         else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    Ok(trench_positions.len())
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_18_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(62));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_18_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
