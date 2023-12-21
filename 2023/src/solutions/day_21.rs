use std::collections::{HashSet, VecDeque};

type Coord = (usize, usize);
type Parsed = (Coord, HashSet<Coord>, usize);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let lines: Vec<Vec<char>> = input_raw
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start = (0, 0);
    let mut walls = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == '#' {
                walls.insert((x, y));
            }
            else if *ch == 'S' {
                start = (x, y);
            }
        }
    }

    let size = lines.len();

    Ok((start, walls, size))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (start, walls, size) = input;
    let plots = run_steps(64, *start, walls, *size);

    Ok(plots)
}

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    let (start, walls, size) = input;

    let steps = 26501365;
    let grid_width = (steps / size) - 1;

    let odd = (((grid_width / 2) * 2) + 1).pow(2);
    let even = (((grid_width + 1) / 2) * 2).pow(2);

    let odd_points = odd * run_steps((steps * 2) + 1, *start, walls, *size);
    let even_points = even * run_steps(steps * 2, *start, walls, *size);

    let end_points = run_steps(size - 1, (start.0, 0), walls, *size)
        + run_steps(size - 1, (start.0, size - 1), walls, *size)
        + run_steps(size - 1, (0, start.1), walls, *size)
        + run_steps(size - 1, (size - 1, start.1), walls, *size);

    let small_diagonal_steps = (size / 2) - 1;
    let small_diagonal_points = (grid_width + 1) * (
        run_steps(small_diagonal_steps, (0, 0), walls, *size)
        + run_steps(small_diagonal_steps, (0, size - 1), walls, *size)
        + run_steps(small_diagonal_steps, (size - 1, 0), walls, *size)
        + run_steps(small_diagonal_steps, (size - 1, size - 1), walls, *size)
    );

    let large_diagonal_steps = ((size * 3) / 2) - 1;
    let large_diagonal_points = grid_width * (
        run_steps(large_diagonal_steps, (0, 0), walls, *size)
        + run_steps(large_diagonal_steps, (0, size - 1), walls, *size)
        + run_steps(large_diagonal_steps, (size - 1, 0), walls, *size)
        + run_steps(large_diagonal_steps, (size - 1, size - 1), walls, *size)
    );

    let points = even_points
        + odd_points
        + end_points
        + small_diagonal_points
        + large_diagonal_points;

    Ok(points)
}

fn run_steps(steps: usize, start: Coord, walls: &HashSet<Coord>, size: usize) -> usize {
    let mut reached_coords = HashSet::new();
    let mut seen = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((start, steps));
    while !queue.is_empty() {
        let (coord, step) = queue.pop_front().unwrap();

        if walls.contains(&coord) {
            continue;
        }
        
        if step % 2 == 0 {
            reached_coords.insert(coord);
        }
        
        if step == 0 || !seen.insert(coord) {
            continue;
        }

        if coord.0 > 0 {
            queue.push_back(((coord.0 - 1, coord.1), step - 1));
        }
        if coord.1 > 0 {
            queue.push_back(((coord.0, coord.1 - 1), step - 1));
        }
        if coord.0 < size - 1 {
            queue.push_back(((coord.0 + 1, coord.1), step - 1));
        }
        if coord.1 < size - 1 {
            queue.push_back(((coord.0, coord.1 + 1), step - 1));
        }
    }

    reached_coords.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_21_1.txt");
        let parsed = parse(example).unwrap();
        let (start, walls, size) = parsed;
        let solution = run_steps(6, start, &walls, size);
        assert_eq!(solution, 16); // on the website is says 16 for 6 steps, would be 42 for 64 steps
    }
}
