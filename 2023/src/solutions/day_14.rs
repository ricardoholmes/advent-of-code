use std::collections::{HashSet, HashMap};

type Coord = (usize, usize);
type Parsed = (Vec<Coord>, HashSet<Coord>, Coord);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let lines: Vec<&str> = input_raw.lines().collect();
    let mut round = vec![];
    let mut cube = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'O' {
                round.push((x, y));
            }
            else if c == '#' {
                cube.insert((x, y));
            }
        }
    }
    
    let size = (lines[0].len(), lines.len());
    Ok((round, cube, size))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (round_rocks, cubes, (_, height)) = input;
    let mut round_rocks = round_rocks.to_owned();

    let mut total = 0;
    for (index, &(x,  y)) in round_rocks.clone().iter().enumerate() {
        let mut stopped = false;
        for i in (0..y).rev() {
            let mut coord = (x, i);
            if round_rocks.contains(&coord) || cubes.contains(&coord) {
                coord.1 += 1;
                round_rocks[index] = coord;
                total += height - coord.1;
                stopped = true;
                break;
            }
        }

        if !stopped {
            round_rocks[index] = (x, 0);
            total += height;
        }
    }

    Ok(total)
}

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    let (
        round_rocks,
        cubes,
        (width, height)
    ) = input;

    let mut round_rocks: HashSet<(usize, usize)> = round_rocks.iter().cloned().collect();
    let mut visited = HashMap::new();

    let cycles_needed = 1_000_000_000;
    let mut cycle = 0;
    while cycle < cycles_needed {
        run_cycle(&mut round_rocks, cubes, *height, *width);
        cycle += 1;

        let mut round_rocks_vec: Vec<Coord> = round_rocks.iter().cloned().collect();
        round_rocks_vec.sort_unstable();

        if let Some(index) = visited.insert(round_rocks_vec, cycle) {
            let cycles_left = (cycles_needed - cycle) % (cycle - index);
            for _ in 0..cycles_left {
                run_cycle(&mut round_rocks, cubes, *height, *width);
            }
            break;
        }
    }

    let mut total = 0;
    for (_, y) in round_rocks {
        total += height - y;
    }

    Ok(total)
}

fn run_cycle(round_rocks: &mut HashSet<Coord>, cubes: &HashSet<Coord>, height: usize, width: usize) {
    let mut moved = true;
    // up
    while moved {
        moved = false;
        for &(x,  y) in round_rocks.clone().iter() {
            let mut new_y = y;
            while new_y > 0 {
                if round_rocks.contains(&(x, new_y - 1)) || cubes.contains(&(x, new_y - 1)) {
                    break;
                }
                new_y -= 1;
            }
            if new_y != y {
                round_rocks.remove(&(x, y));
                round_rocks.insert((x, new_y));
                moved = true;
            }
        }
    }

    moved = true;
    // left
    while moved {
        moved = false;
        for &(x,  y) in round_rocks.clone().iter() {
            let mut new_x = x;
            while new_x > 0 {
                if round_rocks.contains(&(new_x - 1, y)) || cubes.contains(&(new_x - 1, y)) {
                    break;
                }
                new_x -= 1;
            }
            if new_x != x {
                round_rocks.remove(&(x, y));
                round_rocks.insert((new_x, y));
                moved = true;
            }
        }
    }

    moved = true;
    // down
    while moved {
        moved = false;
        for &(x,  y) in round_rocks.clone().iter() {
            let mut new_y = y;
            while new_y < height - 1 {
                if round_rocks.contains(&(x, new_y + 1)) || cubes.contains(&(x, new_y + 1)) {
                    break;
                }
                new_y += 1;
            }
            if new_y != y {
                round_rocks.remove(&(x, y));
                round_rocks.insert((x, new_y));
                moved = true;
            }
        }
    }

    moved = true;
    // right
    while moved {
        moved = false;
        for &(x,  y) in round_rocks.clone().iter() {
            let mut new_x = x;
            while new_x < width - 1 {
                if round_rocks.contains(&(new_x + 1, y)) || cubes.contains(&(new_x + 1, y)) {
                    break;
                }
                new_x += 1;
            }
            if new_x != x {
                round_rocks.remove(&(x, y));
                round_rocks.insert((new_x, y));
                moved = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_14_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(136));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_14_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(64));
    }
}
