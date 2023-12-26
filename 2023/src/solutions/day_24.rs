use crate::safe_unpack;

use std::ops::{Add, Sub, Mul, Div};

type Hailstone = (Vector3, Vector3);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0
    };

    fn parallel_to(&self, other: Self) -> bool {
        if *self == other {
            true
        }
        else if *self == Self::ZERO {
            false
        }
        else {
            let ratio = if self.x != 0.0 {
                other.x / self.x
            } else if self.y != 0.0 {
                other.y / self.y
            } else {
                other.z / self.z
            };

            *self * ratio == other
        }
    }
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

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x)
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
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

pub fn parse(input_raw: &str) -> Result<Vec<Hailstone>, String> {
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

pub fn part_one(hailstones: &[Hailstone]) -> Result<u32, String> {
    let min_boundary = 200000000000000.0;
    let max_boundary = 400000000000000.0;

    get_collisions_in_boundaries(hailstones, min_boundary, max_boundary)
}

pub fn part_two(hailstones: &[Hailstone]) -> Result<i64, String> {
    // remove parallel hailstones
    let hailstones: Vec<Hailstone> = hailstones
        .to_vec()
        .iter()
        .enumerate()
        .filter_map(|(i, &(s, v))| if hailstones.iter().take(i).any(|&(_, other)| v.parallel_to(other)) {
            None
        } else {
            Some((s, v))
        })
        .collect();

    if hailstones.len() < 3 {
        return Err(format!("Not enough non-parallel hailstones."));
    }

    let (s1, v1) = hailstones[0];
    let (s2, v2) = hailstones[1];
    let (s3, v3) = hailstones[2];

    // not enough precision...
    // must implement higher precision int (i256?)
    // note: can be done through a struct holding multiple lower precision ints
    // note: needs support for sqrt too... (t1 is solved through a quadratic equation)
    let a=s1.x;
    let b=v1.x;
    let c=s2.x;
    let d=v2.x;
    let e=s3.x;
    let f=v3.x;
    let g=s1.y;
    let h=v1.y;
    let i=s2.y;
    let j=v2.y;
    let k=s3.y;
    let l=v3.y;
    let m=s1.z;
    let n=v1.z;
    let o=s2.z;
    let p=v2.z;
    let q=s3.z;
    let r=v3.z;

    let a1 = a*k-a*i+c*g-c*k-g*e+i*e;
    let a2 = b*k-b*i+c*h-c*l+f*i-f*k-h*e+l*e;
    let a3 = j*a-l*a-j*e+l*e-d*g+d*k+f*g-f*k;
    let a4 = j*b-l*b-d*h+d*l+f*h-f*j;

    let b1 = a*q-a*o+c*m-c*q-m*e+o*e;
    let b2 = b*q-b*o+c*n-c*r+f*o-f*q-n*e+r*e;
    let b3 = p*a-r*a-p*e+r*e-d*m+d*q+f*m-f*q;
    let b4 = p*b-r*b-d*n+d*r+f*n-f*p;

    let t1 = dbg!(solve_quadratic(a4*b2 - a2*b4, -a1*b4 - a2*b3 + a3*b2 + a4*b1, a3*b1-a1*b3).unwrap()).1;
    let t2 = (a1 + a2*t1)/(a3 + a4*t1);
    let t3 = (t2*(a + t1*b - t1*d) - c*t1 + e*(t1-t2)) / (a - c + t1*b - t1*f - t2*d + t2*f);

    println!("{t1}, {t2}, {t3}");

    let t1 = t1 as f64;
    let t2 = t2 as f64;

    let v_rock = (s1 + t1 * v1 - s2 - t2 * v2) / (t1 - t2);
    let start = s1 + t1 * v1 - t1 * v_rock;

    let ans = start.x + start.y + start.z;

    Ok(ans as i64)
}

fn get_collisions_in_boundaries(hailstones: &[Hailstone], min: f64, max: f64) -> Result<u32, String> {
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
    }

    Ok(collisions)
}

// get the roots of the quadratic equation ax^2 + bx + c = 0
fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant < 0.0 {
        return None;
    }

    Some(((-b + discriminant.sqrt()) / (2.0 * a), (-b - discriminant.sqrt()) / (2.0 * a)))
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
        assert_eq!(solution, Ok(47));
    }
}
