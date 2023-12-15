use std::collections::HashSet;

type Coord = (usize, usize);
type Parsed = (Vec<Coord>, HashSet<Coord>, usize);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let lines = input_raw.lines();
    let mut round = vec![];
    let mut cube = HashSet::new();

    for (y, line) in lines.clone().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'O' {
                round.push((x, y));
            }
            else if c == '#' {
                cube.insert((x, y));
            }
        }
    }

    Ok((round, cube, lines.count()))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (round_rocks, cubes, height) = input;
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
    Ok(0)
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
        assert_eq!(solution, Ok(0));
    }
}
