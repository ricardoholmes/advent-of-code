use std::iter::zip;

pub fn run(input_raw: &str) -> Result<(), String> {
    let solutions = parse(input_raw)?;

    let answer_part_one = part_one(&solutions)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&solutions)?;
    println!("Part two: {}", answer_part_two);

    Ok(())
}

fn parse(input_raw: &str) -> Result<(Vec<(u32, u32)>, (u64, u64)), String> {
    let mut lines = input_raw.lines();

    let mut times_raw: Vec<char> = lines
        .next()
        .unwrap()
        .chars()
        .collect();

    let mut times: Vec<u32> = vec![];
    let mut time_part2: String = String::new();
    loop {
        times_raw = times_raw.iter().skip_while(|c| c.is_numeric()).skip_while(|c| !c.is_numeric()).map(|c| c.to_owned()).collect();
        if times_raw.len() == 0 {
            break;
        }
        time_part2 = format!("{time_part2}{}", times_raw.iter().take_while(|c| c.is_numeric()).map(|c| *c).collect::<String>());
        let time: u32 = times_raw.iter().take_while(|c| c.is_numeric()).collect::<String>().parse().unwrap();
        times.push(time);
    }

    let mut distances_raw: Vec<char> = lines
        .next()
        .unwrap()
        .chars()
        .collect();

    let mut distances: Vec<u32> = vec![];
    let mut distance_part2: String = String::new();
    loop {
        distances_raw = distances_raw.iter().skip_while(|c| c.is_numeric()).skip_while(|c| !c.is_numeric()).map(|c| c.to_owned()).collect();
        if distances_raw.len() == 0 {
            break;
        }
        distance_part2 = format!("{distance_part2}{}", distances_raw.iter().take_while(|c| c.is_numeric()).map(|c| *c).collect::<String>());
        let distance: u32 = distances_raw.iter().take_while(|c| c.is_numeric()).collect::<String>().parse().unwrap();
        distances.push(distance);
    }

    Ok((zip(times, distances).collect(), (time_part2.parse().unwrap(), distance_part2.parse().unwrap())))
}

fn part_one(solutions: &(Vec<(u32, u32)>, (u64, u64))) -> Result<u32, String> {
    let mut total = 1;
    for (time, distance) in solutions.0.clone() {
        let mut min_time = None;
        for i in 1..time {
            if i * (time - i) > distance {
                min_time = Some(i);
                break;
            }
        }
        if let Some(min_time) = min_time {
            let num_ways = (time - min_time + 1) - min_time;
            total *= num_ways;
        }
    }

    Ok(total)
}

fn part_two(solutions: &(Vec<(u32, u32)>, (u64, u64))) -> Result<u64, String> {
    let (time, distance) = solutions.1;

    let mut min_time = None;
    for i in 1..time {
        if i * (time - i) > distance {
            min_time = Some(i);
            break;
        }
    }

    let mut num_ways = None;
    if let Some(min_time) = min_time {
        num_ways = Some((time - min_time + 1) - min_time);
    }

    num_ways.ok_or(String::from("No minimum time found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_6_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_one(&parsed);
        assert_eq!(result, Ok(288));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_6_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_two(&parsed);
        assert_eq!(result, Ok(71503));
    }
}
