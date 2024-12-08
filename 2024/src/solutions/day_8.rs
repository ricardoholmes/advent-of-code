use std::collections::{HashMap, HashSet};

use crate::common::{grid::Point, maths::gcd};

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
    let (w, h, nodes) = input;

    let mut antinodes = HashSet::new();
    for ns in nodes {
        let mut points = vec![];
        for (i, &n1) in ns.iter().enumerate() {
            for &n2 in &ns[i+1..] {
                let divisor = gcd(n1.x - n2.x, n1.y - n2.y);
                let x_offset = (n1.x - n2.x) / divisor;
                let y_offset = (n1.y - n2.y) / divisor;
                points.push((n1, Point::new(x_offset, y_offset)));
            }
        }

        for (p, offset) in points {
            let mut pos = p;
            while pos.x >= 0 && pos.x < *w && pos.y >= 0 && pos.y < *h {
                antinodes.insert(pos);
                pos += offset;
            }

            pos = p;
            while pos.x >= 0 && pos.x < *w && pos.y >= 0 && pos.y < *h {
                antinodes.insert(pos);
                pos -= offset;
            }
        }
    }

    Ok(antinodes.len())
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
        assert_eq!(solution, Ok(34));
    }
}
