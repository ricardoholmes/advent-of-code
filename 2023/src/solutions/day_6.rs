use std::iter::zip;

pub fn run(input_raw: &str) -> Result<(), String> {
    let solutions = parse(input_raw)?;

    let answer_part_one = part_one(&solutions)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&solutions)?;
    println!("Part two: {}", answer_part_two);

    Ok(())
}

fn parse(input_raw: &str) -> Result<(Vec<(u64, u64)>, (u64, u64)), String> {
    let mut lines = input_raw.lines();

    let mut times_raw: Vec<char> = lines
        .next()
        .unwrap()
        .chars()
        .collect();

    let mut times: Vec<u64> = vec![];
    let mut time_part2: String = String::new();
    loop {
        times_raw = times_raw.iter().skip_while(|c| c.is_numeric()).skip_while(|c| !c.is_numeric()).map(|c| c.to_owned()).collect();
        if times_raw.len() == 0 {
            break;
        }
        let time_string: String = times_raw.iter().take_while(|c| c.is_numeric()).map(|c| *c).collect();
        time_part2 = format!("{time_part2}{}", time_string);
        let time: u64 = time_string.parse().unwrap();
        times.push(time);
    }

    let mut distances_raw: Vec<char> = lines
        .next()
        .unwrap()
        .chars()
        .collect();

    let mut distances: Vec<u64> = vec![];
    let mut distance_part2: String = String::new();
    loop {
        distances_raw = distances_raw.iter().skip_while(|c| c.is_numeric()).skip_while(|c| !c.is_numeric()).map(|c| c.to_owned()).collect();
        if distances_raw.len() == 0 {
            break;
        }
        let distance_string: String = distances_raw.iter().take_while(|c| c.is_numeric()).map(|c| *c).collect();
        distance_part2 = format!("{distance_part2}{}", distance_string);
        let distance: u64 = distance_string.parse().unwrap();
        distances.push(distance);
    }

    Ok((zip(times, distances).collect(), (time_part2.parse().unwrap(), distance_part2.parse().unwrap())))
}

fn part_one(solutions: &(Vec<(u64, u64)>, (u64, u64))) -> Result<u64, String> {
    let mut total = 1;
    for (time, distance) in solutions.0.clone() {
        total *= get_num_ways(time, distance);
    }

    Ok(total)
}

fn part_two(solutions: &(Vec<(u64, u64)>, (u64, u64))) -> Result<u64, String> {
    let (time, distance) = solutions.1;

    Ok(get_num_ways(time, distance))
}

fn get_num_ways(time: u64, distance: u64) -> u64 {
    let discriminant = ((time * time) - (4 * distance)) as f64;
    if discriminant < 0.0 {
        return 0;
    }

    let time_for_distance = (time as f64 - discriminant.sqrt()) / 2.0;
    let min_time = (time_for_distance + 1.0) as u64;
    let num_ways = time + 1 - (2 * min_time);

    num_ways
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
