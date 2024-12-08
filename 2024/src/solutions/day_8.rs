use std::{collections::{HashMap, HashSet}, ops::{Add, Sub}};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

type Parsed = (i32, i32, Vec<Vec<Point>>);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut nodes: HashMap<char, Vec<Point>> = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    for line in input_raw.lines() {
        x = 0;
        for c in line.chars() {
            if c != '.' {
                nodes.entry(c).or_default().push(Point::new(x, y));
            }
            x += 1;
        }

        y += 1;
    }

    Ok((x, y, nodes.values().map(|x| x.to_owned()).collect()))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (w, h, nodes) = input;

    let mut antinodes = HashSet::new();
    for ns in nodes {
        let mut points = vec![];
        for (i, &n1) in ns.iter().enumerate() {
            for &n2 in &ns[i+1..] {
                points.push(n1 + n1 - n2);
                points.push(n2 + n2 - n1);
            }
        }

        for p in points {
            if p.x >= 0 && p.x < *w && p.y >= 0 && p.y < *h {
                antinodes.insert(p);
            }
        }
    }

    Ok(antinodes.len())
}

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_8_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(14));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_8_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
