use std::collections::{HashMap, HashSet};

use crate::common::grid::Point;

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
    let ((w, h), walls, mut pos) = input;

    let mut loops = 0;
    let mut positions: HashSet<Point> = HashSet::new();
    let mut direction = Point::new(0, -1);
    while pos.x >= 0 && pos.x < *w && pos.y >= 0 && pos.y < *h {
        positions.insert(pos);

        let new_pos = pos + direction;
        if walls.contains(&new_pos) {
            direction = direction.rotate90();
        }
        else {
            if !positions.contains(&new_pos) {
                let mut walls_with_obstruction = walls.clone();
                walls_with_obstruction.insert(new_pos);
                if check_for_loop(pos, direction, &walls_with_obstruction, *w, *h) {
                    loops += 1;
                }
            }
            pos = new_pos;
        }
    }

    Ok(loops)
}

fn check_for_loop(pos: Point, direction: Point, walls: &HashSet<Point>, w: i32, h: i32) -> bool {
    let mut pos = pos;
    let mut direction = direction;

    // key = pos, value = directions used to reach that point
    let mut positions: HashMap<Point, HashSet<Point>> = HashMap::new();
    while pos.x >= 0 && pos.x < w && pos.y >= 0 && pos.y < h {
        let entry = positions.entry(pos).or_default();
        if !entry.insert(direction) { // insert returns false if value was already present
            return true;
        }

        let new_pos = pos + direction;
        if walls.contains(&new_pos) {
            direction = direction.rotate90();
        }
        else {
            pos = new_pos;
        }
    }

    false
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
        assert_eq!(solution, Ok(6));
    }
}
