use std::ops::{Add, Sub, Mul, Div};

use crate::safe_unpack;

type Parsed = (Vector3, Vector3);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut hailstones = vec![];
    for line in input_raw.lines() {
        let line = line.replace("  ", " "); // remove double spaces (present in example)
        safe_unpack!(line.split(" @ "), position, velocity);
        let mut position_split = position.split(", ");
        let position = (
            position_split.next().unwrap().parse().unwrap(),
            position_split.next().unwrap().parse().unwrap(),
            position_split.next().unwrap().parse().unwrap()
        );

        let mut velocity_split = velocity.split(", ");
        let velocity = (
            velocity_split.next().unwrap().parse().unwrap(),
            velocity_split.next().unwrap().parse().unwrap(),
            velocity_split.next().unwrap().parse().unwrap()
        );

        hailstones.push((position.into(), velocity.into()));
    }

    Ok(hailstones)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let min_boundary = 200000000000000.0;
    let max_boundary = 400000000000000.0;

    get_collisions_in_boundaries(input, min_boundary, max_boundary)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

fn get_collisions_in_boundaries(hailstones: &[Parsed], min: f64, max: f64) -> Result<usize, String> {
    let mut collisions = 0;
    for (n, (s1, v1)) in hailstones.iter().enumerate() {
        let s1 = *s1;
        let Vector3 { x: x1, y: y1, z: _ } = s1;
        let v1 = *v1;

        // skip to the (n+1)th hailstone so that
        // - no hailstone is compared to itself
        // - collisions for any two hailstones are only calculated once
        //   - e.g. stone 1 will be checked for stone 0 BUT stone 0 will not be checked for stone 1
        for &(s2, v2) in hailstones.iter().skip(n + 1) {
            let Vector3 { x: x2, y: y2, z: _ } = s2;

            // assuming no two lines are the same,
            // if lines are parallel then they won't collide
            if v1.x * v2.y == v2.x * v1.y {
                continue;
            }

            // result of solving the simultaneous equations derived from s1 + (t1 * v1) = s2 + (t2 * v2)
            let t1 = (((x1 - x2) * v2.y) - ((y1 - y2) * v2.x)) / ((v1.y * v2.x) - (v1.x * v2.y));
            let t2 = ((y1 - y2) + (t1 * v1.y)) / v2.y;

            // if collisions are in the past
            if t1 < 0.0 || t2 < 0.0 {
                continue;
            }

            // find collision
            let collision = s1 + (t1 * v1);

            // if collision is out of bounds
            if collision.x < min || collision.x > max || collision.y < min || collision.y > max {
                continue;
            }

            collisions += 1;
        }
        panic!();
    }

    Ok(collisions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_24_1.txt");
        let parsed = parse(example).unwrap();
        let solution = get_collisions_in_boundaries(&parsed, 7.0, 27.0);
        assert_eq!(solution, Ok(2));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_24_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
