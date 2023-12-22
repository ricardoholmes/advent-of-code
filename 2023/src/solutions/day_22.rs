use std::collections::HashSet;

use crate::safe_unpack;

type Cube = (usize, usize, usize);
type Parsed = Vec<Cube>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut bricks = vec![];
    for line in input_raw.lines() {
        let mut brick = vec![];
        safe_unpack!(
            line.split('~'),
            start,
            end
        );

        let mut start = start
            .split(',')
            .map(|n| n
                .parse()
                .unwrap()
            );

        let mut end = end
            .split(',')
            .map(|n| n
                .parse()
                .unwrap()
            );

        let start_cube = (start.next().unwrap(), start.next().unwrap(), start.next().unwrap());
        let end_cube = (end.next().unwrap(), end.next().unwrap(), end.next().unwrap());

        for x in start_cube.0..=end_cube.0 {
            for y in start_cube.1..=end_cube.1 {
                for z in start_cube.2..=end_cube.2 {
                    brick.push((x, y, z));
                }
            }
        }
        if brick.len() == 1 && start_cube != end_cube {
            panic!();
        }

        bricks.push(brick);
    }

    Ok(bricks)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut bricks = input.to_vec();
    bricks.sort_unstable_by(|a, b| a[0].2.cmp(&b[0].2));

    let mut moved = true;
    while moved {
        moved = false;
        for index in 0..bricks.len() {
            let mut z_offset = 0;
            let brick = &bricks[index];
            while brick[0].2 > z_offset {
                let mut invalid_offset = false;
                let maybe_offset = z_offset + 1;
                for other_index in 0..bricks.len() {
                    if other_index == index {
                        continue;
                    }
                    let other_brick = &bricks[other_index];
                    for cube in brick {
                        if other_brick.contains(&(cube.0, cube.1, cube.2 - maybe_offset)) {
                            invalid_offset = true;
                            break;
                        }
                    }
                }
                if invalid_offset {
                    break;
                }
                z_offset = maybe_offset;
            }
            let mut new_brick = brick.clone();
            for cube in new_brick.iter_mut() {
                cube.2 -= z_offset;
            }
            bricks[index] = new_brick;

            if z_offset > 0 {
                moved = true;
            }
        }
    }

    let mut vital_bricks = HashSet::new();
    for (index, brick) in bricks.iter().enumerate() {
        if vital_bricks.contains(&index) {
            continue;
        }

        let mut bricks_below = vec![];
        for (other_index, other_brick) in bricks.iter().enumerate() {
            if index == other_index {
                continue;
            }

            for cube in other_brick {
                let mut cube_above = cube.clone();
                cube_above.2 += 1;

                if brick.contains(&cube_above) {
                    bricks_below.push(other_index);
                    break;
                }
            }
        }

        if bricks_below.len() == 1 {
            vital_bricks.insert(bricks_below[0]);
        }
    }

    let destructable = bricks.len() - vital_bricks.len();

    Ok(destructable)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_22_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(5));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_22_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
