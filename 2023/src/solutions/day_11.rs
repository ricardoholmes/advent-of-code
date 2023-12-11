use std::collections::HashSet;

type Coord = (usize, usize);

pub fn parse(input_raw: &str) -> Result<(Vec<Coord>, Vec<usize>, Vec<usize>), String> {
    let lines = input_raw.lines();

    let mut stars = vec![];
    let mut empty_y = vec![];
    let mut empty_x: HashSet<usize> = lines
        .clone()
        .next()
        .ok_or(format!("Input string is empty"))?
        .chars()
        .enumerate()
        .map(|(x, _)| x)
        .collect();

    for (y, line) in lines.enumerate() {
        if line.find(|c| c == '#').is_none() {
            empty_y.push(y);
            continue;
        }

        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                stars.push((x, y));
                empty_x.remove(&x);
            }
        }
    }

    let empty_x: Vec<usize> = empty_x
        .iter()
        .map(|x| *x)
        .collect();

    Ok((stars, empty_x, empty_y))
}

pub fn part_one(input: &(Vec<Coord>, Vec<usize>, Vec<usize>)) -> Result<usize, String> {
    let (stars, empty_x, empty_y) = input;

    let mut stars = stars.clone();
    expand(&mut stars, empty_x.clone(), empty_y.clone(), 2)?;

    let total_dist = count_total_distances(&stars);

    Ok(total_dist)
}

pub fn part_two(input: &(Vec<Coord>, Vec<usize>, Vec<usize>)) -> Result<usize, String> {
    let (stars, empty_x, empty_y) = input;

    let mut stars = stars.clone();
    expand(&mut stars, empty_x.clone(), empty_y.clone(), 1_000_000)?;

    let total_dist = count_total_distances(&stars);

    Ok(total_dist)
}

fn count_total_distances(stars: &[Coord]) -> usize {
    let mut total_dist = 0;

    for (i, star) in stars.iter().enumerate() {
        for other_star in stars.iter().skip(i + 1) {
            if star == other_star {
                continue;
            }
            let total_dist_x = other_star.0 as i32 - star.0 as i32;
            let total_dist_y = other_star.1 as i32 - star.1 as i32;

            total_dist += total_dist_x.abs() as usize + total_dist_y.abs() as usize
        }
    }

    total_dist
}

fn expand(stars: &mut Vec<Coord>, mut empty_x: Vec<usize>, mut empty_y: Vec<usize>, size_of_empty: usize) -> Result<(), String> {
    empty_x.sort_unstable();
    empty_x.reverse();
    empty_y.reverse();

    if size_of_empty == 0 {
        return Err(format!("Size of expanded empty lines must be >= 1."));
    }

    for star in stars.iter_mut() {
        for &x in &empty_x {
            if star.0 > x {
                star.0 += size_of_empty - 1;
            }
        }
        for &y in &empty_y {
            if star.1 > y {
                star.1 += size_of_empty - 1;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_11_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(374));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_11_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(82000210));
    }
}
