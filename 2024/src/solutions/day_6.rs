use std::{collections::HashSet, ops::Add};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn rotate90(self) -> Self {
        Point {
            x: -self.y,
            y: self.x
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

type Parsed = ((i32, i32), HashSet<Point>, Point);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut walls = HashSet::new();
    let mut player = Point::new(0, 0);

    let mut x = 0;
    let mut y = 0;
    for line in input_raw.lines() {
        x = 0;
        for c in line.chars() {
            match c {
                '#' => { walls.insert(Point::new(x, y)); }
                '^' => { player = Point::new(x, y); }
                _ => ()
            }

            x += 1;
        }

        y += 1;
    }

    Ok(((x, y), walls, player))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let ((w, h), walls, mut pos) = input;

    let mut direction = Point::new(0, -1);
    let mut positions = HashSet::new();
    while pos.x >= 0 && pos.x < *w && pos.y >= 0 && pos.y < *h {
        positions.insert(pos);

        let new_pos = pos + direction;
        if walls.contains(&new_pos) {
            direction = direction.rotate90();
        }
        else {
            pos = new_pos;
        }
    }

    Ok(positions.len())
}

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_6_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(41));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_6_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
