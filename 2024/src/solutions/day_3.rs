extern crate regex;

use regex::Regex;

type Parsed = String;

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    Ok(input_raw.to_string())
}

pub fn part_one(input: &Parsed) -> Result<u32, String> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut total: u32 = 0;
    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        total += x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap();
    }
    Ok(total)
}

pub fn part_two(input: &Parsed) -> Result<u32, String> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut total: u32 = 0;

    let mut start = Some(0);
    let mut stop_idx;
    while start.is_some() {
        let start_idx = start.unwrap();
        stop_idx = input[start_idx..].find("don't()").and_then(|x| Some(x + start_idx)).unwrap_or(input.len());
        for (_, [x, y]) in re.captures_iter(&input[start_idx..stop_idx]).map(|c| c.extract()) {
            total += x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap();
        }
        start = input[stop_idx..].find("do()").and_then(|x| Some(x + stop_idx));
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_3_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(161));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_3_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(48));
    }
}
