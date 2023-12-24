extern crate z3;

use crate::safe_unpack;

use std::ops::{Add, Sub, Mul, Div, Rem};
use self::z3::{ ast::{ Ast, Int }, SatResult };

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

    fn all(&self, pred: fn(f64) -> bool) -> bool {
        pred(self.x) && pred(self.y) && pred(self.z)
    }

    fn any(&self, pred: fn(f64) -> bool) -> bool {
        pred(self.x) || pred(self.y) || pred(self.z)
    }

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

            self.x * ratio == other.x && self.y * ratio == other.y && self.z * ratio == other.z
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
    // let hailstones: Vec<Hailstone> = hailstones
    //     .to_vec()
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(i, &(s, v))| if hailstones.iter().take(i).any(|&(_, other)| v.parallel_to(other)) {
    //         None
    //     } else {
    //         Some((s, v))
    //     })
    //     .collect();

    // if hailstones.len() < 3 {
    //     return Err(format!("Not enough non-parallel hailstones."));
    // }

    // let (s_i, v_i) = hailstones[0];
    // let (s_j, v_j) = hailstones[1];
    // let (s_k, v_k) = hailstones[2];

    // println!("{} + {}a + ((c - a) * ({} + {}a - {} - {}b) / (a - b))) - {} - {}c = 0", s_i.x, v_i.x, s_i.x, v_i.x, s_j.x, v_j.x, s_k.x, v_k.x);
    // println!("{} + {}a + ((c - a) * ({} + {}a - {} - {}b) / (a - b))) - {} - {}c = 0", s_i.y, v_i.y, s_i.y, v_i.y, s_j.y, v_j.y, s_k.y, v_k.y);
    // println!("{} + {}a + ((c - a) * ({} + {}a - {} - {}b) / (a - b))) - {} - {}c = 0", s_i.z, v_i.z, s_i.z, v_i.z, s_j.z, v_j.z, s_k.z, v_k.z);

    let context = z3::Context::new(&z3::Config::new());
    let solver  = z3::Solver::new(&context);

    let rsx = Int::new_const(&context, "rsx");
    let rsy = Int::new_const(&context, "rsy");
    let rsz = Int::new_const(&context, "rsz");

    let rvx = Int::new_const(&context, "rvx");
    let rvy = Int::new_const(&context, "rvy");
    let rvz = Int::new_const(&context, "rvz");

    let zero = Int::from_i64(&context, 0);
    for (i, (s, v)) in hailstones.iter().enumerate() {
        let t = Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.ge(&zero));

        let sx = Int::from_i64(&context, s.x as i64);
        let sy = Int::from_i64(&context, s.y as i64);
        let sz = Int::from_i64(&context, s.z as i64);

        let vx = Int::from_i64(&context, v.x as i64);
        let vy = Int::from_i64(&context, v.y as i64);
        let vz = Int::from_i64(&context, v.z as i64);

        solver.assert(&(&sx + &t * &vx)._eq(&(&rsx + &t * &rvx)));
        solver.assert(&(&sy + &t * &vy)._eq(&(&rsy + &t * &rvy)));
        solver.assert(&(&sz + &t * &vz)._eq(&(&rsz + &t * &rvz)));
    }

    let answer = if solver.check() == SatResult::Sat {
        solver
            .get_model()
            .unwrap()
            .eval(&(&rsx + &rsy + &rsz), true)
            .unwrap()
            .as_i64()
            .unwrap()
    } else {
        return Err(format!("Failed to solve."))
    };

    Ok(answer)

    // let t1 = Int::new_const(&context, "t1");
    // let t2 = Int::new_const(&context, "t2");
    // let t3 = Int::new_const(&context, "t3");

    // let ax = Int::from_i64(&context, s_i.x as i64);
    // let bx = Int::from_i64(&context, v_i.x as i64);
    // let ay = Int::from_i64(&context, s_i.y as i64);
    // let by = Int::from_i64(&context, v_i.y as i64);
    // let az = Int::from_i64(&context, s_i.z as i64);
    // let bz = Int::from_i64(&context, v_i.z as i64);
    
    // let cx = Int::from_i64(&context, s_j.x as i64);
    // let dx = Int::from_i64(&context, v_j.x as i64);
    // let cy = Int::from_i64(&context, s_j.y as i64);
    // let dy = Int::from_i64(&context, v_j.y as i64);
    // let cz = Int::from_i64(&context, s_j.z as i64);
    // let dz = Int::from_i64(&context, v_j.z as i64);

    // let ex = Int::from_i64(&context, s_k.x as i64);
    // let fx = Int::from_i64(&context, v_k.x as i64);
    // let ey = Int::from_i64(&context, s_k.y as i64);
    // let fy = Int::from_i64(&context, v_k.y as i64);
    // let ez = Int::from_i64(&context, s_k.z as i64);
    // let fz = Int::from_i64(&context, v_k.z as i64);

    // let zero = Int::from_i64(&context, 0);

    // solver.assert(&(&ax + &(&bx * &t1) + &(&(&t3 - &t1) * &(&(&ax + &(&bx * &t1) - &cx - &(&t2 * &dx)) / &(&t1 - &t2))) - &ex - &(&fx * &t3))._eq(&zero));
    // solver.assert(&(&ay + &(&by * &t1) + &(&(&t3 - &t1) * &(&(&ay + &(&by * &t1) - &cy - &(&t2 * &dy)) / &(&t1 - &t2))) - &ey - &(&fy * &t3))._eq(&zero));
    // solver.assert(&(&az + &(&bz * &t1) + &(&(&t3 - &t1) * &(&(&az + &(&bz * &t1) - &cz - &(&t2 * &dz)) / &(&t1 - &t2))) - &ez - &(&fz * &t3))._eq(&zero));

    // println!("finished asserting");
    // let (t1, t2) = if solver.check() == SatResult::Sat {
    //     (
    //         solver
    //             .get_model()
    //             .unwrap()
    //             .eval(&t1, true)
    //             .unwrap()
    //             .as_i64()
    //             .unwrap() as f64,

    //         solver
    //             .get_model()
    //             .unwrap()
    //             .eval(&t2, true)
    //             .unwrap()
    //             .as_i64()
    //             .unwrap() as f64
    //     )
    // } else {
    //     return Err(format!("Failed to solve."))
    // };
    // println!("{t1}, {t2}");

    // let v_rock = (s_i + t1 * v_i - s_j - t2 * v_j) / (t1 - t2);
    // let start = s_i + t1 * v_i - t1 * v_rock;

    // let a = s_i.x;
    // let b = v_i.x;
    // let c = s_j.x;
    // let d = v_j.x;
    // let e = s_k.x;
    // let f = v_k.x;

    // let g = s_i.y;
    // let h = v_i.y;
    // let i = s_j.y;
    // let j = v_j.y;
    // let k = s_k.y;
    // let l = v_k.y;


    // let a1 = ((d-b)*t1 + e-a)*(-j-l);
    // let a2 = ((j-h)*t1 + k-g)*(-d-f);
    // let b1 = ((d-b)*t1 + e-a)*((l-h)*t1 + -g-i) + (c-e)*(-j-l);
    // let b2 = ((j-h)*t1 + k-g)*((f-b)*t1 + -a-c) + (i-k)*(-d-f);
    // let c1 = (c-e)*((l-h)*t1 + -g-i)*t1;
    // let c2 = (i-k)*((f-b)*t1 + -a-c)*t1;

    // let t2 = if let Some(t) = solve_quadratic(a1-a2, b1-b2, c1-c2) {
    //     println!("{t:?}");
    //     t.0
    // } else {
    //     -1.0
    // };

    // let t3 = ((c - e) * t1 + ((d - b) * t1 + (e - a)) * t2) / ((f - b) * t1 + (-d-f) * t2 + (-a-c));

    // println!("{t1}, {t2}, {t3}");

    // below is my suffering
    // 'outer: loop {
    //     t_i += 1.0;
    //     let s_i = s_i + (t_i * v_i);

    //     if t_i > 10.0 {
    //         panic!();
    //     }

    //     // let mut t_j = 0.0;
    //     // loop {
    //     //     t_j += 1.0;

    //     //     // prevent zero division
    //     //     if t_j == t_i {
    //     //         continue;
    //     //     }

    //     //     let s_j = s_j + (t_j * v_j);
    //     //     let trajectory = s_j - s_i;
    //     //     let v_rock = trajectory / (t_j - t_i);

    //     //     // if any components are less than 1
    //     //     if v_rock.any(|n| n.abs() < 1.0) {
    //     //         println!("less than 1");
    //     //         if t_j < t_i {
    //     //             continue;
    //     //         }
    //     //         break;
    //     //     }

    //     //     // if not all components are integers
    //     //     if !v_rock.all(|n| n.trunc() == n) {
    //     //         print!("t_j: {t_j} -> ");
    //     //         t_j += lcm(trajectory.x, lcm(trajectory.y, trajectory.z));
    //     //         println!("{t_j}");
    //     //         continue;
    //     //     }

    //     //     // if it reaches here, all components of velocity are integers and >= 1
    //     //     start = s_i - (t_i * v_rock);

    //     //     // println!("\n{t_i} - {t_j}");
    //     //     // println!("start:    {start:?}");
    //     //     // println!("velocity: {v_rock:?}");

    //     //     let mut all_hailstones_hit = true;
    //     //     for &(s, v) in hailstones.iter().skip(2) {
    //     //         if v_rock * v == Vector3::ZERO {
    //     //             // println!("lines are parallel");
    //     //             all_hailstones_hit = false;
    //     //             break;
    //     //         }

    //     //         // result of solving the simultaneous equations derived from start + (t * v_rock) = s + (t * v)
    //     //         let s_diff = s - start;
    //     //         let v_diff = v_rock - v;
    //     //         let t = if v_diff.x != 0.0 {
    //     //             s_diff.x / v_diff.x
    //     //         } else if v_diff.y != 0.0 {
    //     //             s_diff.y / v_diff.y
    //     //         } else {
    //     //             s_diff.z / v_diff.z
    //     //         };

    //     //         if t < 0.0 {
    //     //             // println!("hit at bad time {t}");
    //     //             all_hailstones_hit = false;
    //     //             break;
    //     //         }

    //     //         let collision = start + (t * v_rock);
    //     //         if !collision.all(|n| n.trunc() == n) {
    //     //             // println!("hit at bad position {t}");
    //     //             all_hailstones_hit = false;
    //     //             break;
    //     //         }
    //     //     }

    //     //     if all_hailstones_hit {
    //     //         break 'outer;
    //     //     }
    //     // }
    // }
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

fn lcm<T>(a: T, b: T) -> T where T: Copy + Mul<Output=T> + PartialEq + Div<Output=T> + Rem<Output=T> + Default {
    a * b / gcd(a, b)
}

fn gcd<T>(a: T, b: T) -> T where T: Copy + PartialEq + Rem<Output=T> + Default {
    if b == T::default() {
        a
    }
    else {
        gcd(b, a % b)
    }
}

// get the positive root of the quadratic equation ax^2 + bx + c = 0
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
