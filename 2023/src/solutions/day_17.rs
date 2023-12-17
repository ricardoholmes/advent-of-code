use std::collections::{VecDeque, HashSet};

type Parsed = Vec<usize>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        *self == Direction::Left || *self == Direction::Right
    }

    // fn is_vertical(&self) -> bool {
    //     *self == Direction::Up || *self == Direction::Down
    // }
}

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let lines = input_raw
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        )
        .collect();

    Ok(lines)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut movements = VecDeque::new();

    let mut crossed_horizontal = 0;
    let mut crossed_vertical = 0;
    for i in 1..=3 {
        crossed_horizontal += input[0][i];
        match movements.iter().position(|&(_, _, crossed, _)| crossed > crossed_horizontal) {
            Some(index) => movements.insert(index, ((i, 0), Direction::Right, crossed_horizontal, HashSet::new())),
            None => movements.push_back(((i, 0), Direction::Right, crossed_horizontal, HashSet::new())),
        }

        crossed_vertical += input[i][0];
        match movements.iter().position(|&(_, _, crossed, _)| crossed > crossed_vertical) {
            Some(index) => movements.insert(index, ((0, i), Direction::Down, crossed_vertical, HashSet::new())),
            None => movements.push_back(((0, i), Direction::Down, crossed_vertical, HashSet::new())),
        }
    }

    let border_x = input[0].len() - 1;
    let border_y = input.len() - 1;

    let mut expanded = HashSet::new();
    while !movements.is_empty() {
        let movement = movements.pop_front().unwrap();
        let (pos, direction, crossed, mut coords) = movement;
        coords.insert(pos);

        if pos == (border_x, border_y) {
            // for y in 0..=border_y {
            //     for x in 0..=border_x {
            //         if coords.contains(&(x, y)) {
            //             print!("#");
            //         }
            //         else {
            //             print!(".");
            //         }
            //     }
            //     println!();
            // }
            return Ok(crossed);
        }

        if !expanded.insert((pos, direction)) {
            continue;
        }

        let mut first_dir_crossed = crossed;
        let mut second_dir_crossed = crossed;
        for distance in 1..=3 {
            if direction.is_horizontal() {
                if distance <= pos.1 {
                    let new_pos = (pos.0, pos.1 - distance);
                    let new_direction = Direction::Up;
                    first_dir_crossed += input[new_pos.1][new_pos.0];
                    match movements.iter().position(|&(_, _, crossed, _)| crossed > first_dir_crossed) {
                        Some(index) => movements.insert(index, (new_pos, new_direction, first_dir_crossed, coords.clone())),
                        None => movements.push_back((new_pos, new_direction, first_dir_crossed, coords.clone())),
                    }
                }

                if distance + pos.1 <= border_y {
                    let new_pos = (pos.0, pos.1 + distance);
                    let new_direction = Direction::Down;
                    second_dir_crossed += input[new_pos.1][new_pos.0];
                    match movements.iter().position(|&(_, _, crossed, _)| crossed > second_dir_crossed) {
                        Some(index) => movements.insert(index, (new_pos, new_direction, second_dir_crossed, coords.clone())),
                        None => movements.push_back((new_pos, new_direction, second_dir_crossed, coords.clone())),
                    }
                }
            }
            else {
                if distance <= pos.0 {
                    let new_pos = (pos.0 - distance, pos.1);
                    let new_direction = Direction::Left;
                    first_dir_crossed += input[new_pos.1][new_pos.0];
                    match movements.iter().position(|&(_, _, crossed, _)| crossed > first_dir_crossed) {
                        Some(index) => movements.insert(index, (new_pos, new_direction, first_dir_crossed, coords.clone())),
                        None => movements.push_back((new_pos, new_direction, first_dir_crossed, coords.clone())),
                    }
                }

                if distance + pos.0 <= border_x {
                    let new_pos = (pos.0 + distance, pos.1);
                    let new_direction = Direction::Right;
                    second_dir_crossed += input[new_pos.1][new_pos.0];
                    match movements.iter().position(|&(_, _, crossed, _)| crossed > second_dir_crossed) {
                        Some(index) => movements.insert(index, (new_pos, new_direction, second_dir_crossed, coords.clone())),
                        None => movements.push_back((new_pos, new_direction, second_dir_crossed, coords.clone())),
                    }
                }
            }
        }
    }

    Err(format!("End not reached"))
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_17_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(102));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_17_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
