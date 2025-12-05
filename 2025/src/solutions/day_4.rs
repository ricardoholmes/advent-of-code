use std::collections::HashSet;

use crate::common::grid::Point;

type Parsed = HashSet<Point>;

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut output = HashSet::new();
    for (y, line) in input_raw.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '@' {
                output.insert(Point::new(x as i32, y as i32));
            }
        }
    }

    Ok(output)
}

pub fn part_one(input: &Parsed) -> Result<u32, String> {
    let mut out = 0;
    for &point in input {
        let mut count = 0;
        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                if y_offset == 0 && x_offset == 0 {
                    continue;
                }

                let offset = Point::new(x_offset, y_offset);
                if input.contains(&(point + offset)) {
                    count += 1;
                }
            }
        }
        if count < 4 {
            out += 1;
        }
    }
    Ok(out)
}

pub fn part_two(input: &Parsed) -> Result<u64, String> {
    let mut map = input.clone();
    let mut out = 0;
    loop {
        let mut to_remove = vec![];
        for &point in &map {
            let mut count = 0;
            for y_offset in -1..=1 {
                for x_offset in -1..=1 {
                    if y_offset == 0 && x_offset == 0 {
                        continue;
                    }
                    
                    let offset = Point::new(x_offset, y_offset);
                    if map.contains(&(point + offset)) {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                to_remove.push(point);
                out += 1;
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for point in to_remove {
            map.remove(&point);
        }
    }

    Ok(out)
}
