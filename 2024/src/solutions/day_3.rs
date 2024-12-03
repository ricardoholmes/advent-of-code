extern crate regex;

use regex::Regex;

#[derive(Clone, Copy, Debug)]
pub enum Token {
    Mul(u32),
    Do,
    Dont
}

type Parsed = Token;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let re = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();

    let mut out = vec![];
    for cap in re.captures_iter(input_raw) {
        if cap.name("do").is_some() {
            out.push(Token::Do)
        }
        else if cap.name("dont").is_some() {
            out.push(Token::Dont)
        }
        else { // must be mul
            let x = cap.name("x").unwrap().as_str().parse::<u32>().unwrap();
            let y = cap.name("y").unwrap().as_str().parse::<u32>().unwrap();
            out.push(Token::Mul(x * y))
        }
    }

    Ok(out)
}

pub fn part_one(input: &[Parsed]) -> Result<u32, String> {
    let mut total = 0;
    for tok in input {
        if let Token::Mul(x) = tok {
            total += x;
        }
    }
    Ok(total)
}

pub fn part_two(input: &[Parsed]) -> Result<u32, String> {
    let mut total = 0;
    let mut enabled = true;
    for &tok in input {
        match tok {
            Token::Do => enabled = true,
            Token::Dont => enabled = false,
            Token::Mul(x) => if enabled { total += x },
        }
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
