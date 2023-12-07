use std::iter::zip;

pub fn parse(input_raw: &str) -> Result<(Vec<(u64, u64)>, (u64, u64)), String> {
    let mut lines_parsed = input_raw
        .lines()
        .map(|line| line
            .split_ascii_whitespace()
            .filter(|s| !s.is_empty())
            .skip(1)
        );

    let times_line = lines_parsed.next().unwrap();
    let times = times_line.clone().map(|n| n.parse::<u64>().unwrap());
    let time_part2: String = times_line.collect();

    let distances_line = lines_parsed.next().unwrap();
    let distances = distances_line.clone().map(|n| n.parse::<u64>().unwrap());
    let distance_part2: String = distances_line.collect();

    Ok((zip(times, distances).collect(), (time_part2.parse().unwrap(), distance_part2.parse().unwrap())))
}

pub fn part_one(solutions: &(Vec<(u64, u64)>, (u64, u64))) -> Result<u64, String> {
    let mut total = 1;
    for (time, distance) in solutions.0.clone() {
        total *= get_num_ways(time, distance);
    }

    Ok(total)
}

pub fn part_two(solutions: &(Vec<(u64, u64)>, (u64, u64))) -> Result<u64, String> {
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
