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

    let mut empty_x: Vec<usize> = empty_x
        .iter()
        .map(|x| *x)
        .collect();

    empty_x.sort_unstable();
    empty_x.reverse();

    empty_y.reverse();

    Ok((stars, empty_x, empty_y))
}

pub fn part_one(input: &(Vec<Coord>, Vec<usize>, Vec<usize>)) -> Result<usize, String> {
    let (stars, empty_x, empty_y) = input;
    let mut stars = stars.clone();
    expand(&mut stars, empty_x, empty_y, 2);

    let mut total_dist = 0;
    let mut x_max = 0;
    let mut y_max = 0;

    for (i, star) in stars.iter().enumerate() {
        if star.0 > x_max {
            x_max = star.0;
        }
        if star.1 > y_max {
            y_max = star.1;
        }

        for other_star in stars.iter().skip(i + 1) {
            if star == other_star {
                continue;
            }
            let total_dist_x = other_star.0 as i32 - star.0 as i32;
            let total_dist_y = other_star.1 as i32 - star.1 as i32;

            total_dist += total_dist_x.abs() as usize + total_dist_y.abs() as usize
        }
    }

    // for y in 0..=y_max {
    //     for x in 0..=x_max {
    //         if stars.contains(&(x, y)) {
    //             print!("{}", stars.iter().position(|&star| star == (x, y)).unwrap() + 1);
    //         }
    //         else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    Ok(total_dist)
}

pub fn part_two(input: &(Vec<Coord>, Vec<usize>, Vec<usize>)) -> Result<usize, String> {
    let (stars, empty_x, empty_y) = input;

    let mut stars = stars.clone();
    expand(&mut stars, empty_x, empty_y, 1_000_000);

    let mut total_dist = 0;
    let mut x_max = 0;
    let mut y_max = 0;

    for (i, star) in stars.iter().enumerate() {
        if star.0 > x_max {
            x_max = star.0;
        }
        if star.1 > y_max {
            y_max = star.1;
        }

        for other_star in stars.iter().skip(i + 1) {
            if star == other_star {
                continue;
            }
            let total_dist_x = other_star.0 as i32 - star.0 as i32;
            let total_dist_y = other_star.1 as i32 - star.1 as i32;

            total_dist += total_dist_x.abs() as usize + total_dist_y.abs() as usize
        }
    }

    // for y in 0..=y_max {
    //     for x in 0..=x_max {
    //         if stars.contains(&(x, y)) {
    //             print!("{}", stars.iter().position(|&star| star == (x, y)).unwrap() + 1);
    //         }
    //         else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    Ok(total_dist)
}

fn expand(stars: &mut Vec<Coord>, empty_x: &[usize], empty_y: &[usize], size_of_empty: usize) {
    for star in stars.iter_mut() {
        for &x in empty_x {
            if star.0 > x {
                star.0 += size_of_empty - 1;
            }
        }
        for &y in empty_y {
            if star.1 > y {
                star.1 += size_of_empty - 1;
            }
        }
    }
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
