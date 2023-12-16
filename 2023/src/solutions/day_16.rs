use std::collections::HashSet;

type Parsed = Vec<char>;
type Coord = (i32, i32);

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

    fn is_vertical(&self) -> bool {
        *self == Direction::Up || *self == Direction::Down
    }
}

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let lines = input_raw
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Ok(lines)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut energized = HashSet::new();
    let mut past_movements = HashSet::new();
    let mut movements: Vec<(Coord, Direction)> = vec![((0, 0), Direction::Right)];

    while !movements.is_empty() {
        let (pos, mut direction) = movements.pop().unwrap();

        if !past_movements.insert((pos, direction)) {
            continue;
        }

        if pos.0 < 0 || pos.0 >= input[0].len() as i32 {
            continue;
        }

        if pos.1 < 0 || pos.1 >= input.len() as i32 {
            continue;
        }

        energized.insert(pos);

        let node = input[pos.1 as usize][pos.0 as usize];
        if node == '-' && direction.is_vertical() {
            movements.push(((pos.0 - 1, pos.1), Direction::Left));
            movements.push(((pos.0 + 1, pos.1), Direction::Right));
            continue;
        }
        else if node == '|' && direction.is_horizontal() {
            movements.push(((pos.0, pos.1 - 1), Direction::Up));
            movements.push(((pos.0, pos.1 + 1), Direction::Down));
            continue;
        }
        else if node == '/' {
            direction = match direction {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            };
        }
        else if node == '\\' {
            direction = match direction {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            };
        }

        movements.push((match direction {
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
        }, direction));
    }

    Ok(energized.len())
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_16_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(46));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_16_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
