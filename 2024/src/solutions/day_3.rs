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

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    Ok(0)
}
