extern crate regex;

use self::regex::Regex;

pub fn run() -> Result<(), String> {
    let input_str = include_str!("../../inputs/input_1.txt");

    let input_str = input_str
        .strip_suffix('\n')
        .unwrap_or(input_str)
        .to_string();

    match part_one(&input_str) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };
    part_two(&input_str)
}

fn part_one(input_str: &String) -> Result<(), String> {
    let input: Vec<u32> = input_str
        .split("\n")
        .map(|s|
            {
                let digits = s.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();

                let num: String = format!("{}{}", digits.chars().nth(0).unwrap(), digits.chars().nth(digits.len()-1).unwrap());

                num.parse().expect("Failed to parse")
            })
        .collect();

    let sum = input.iter().fold(0, |total, x| total + x);
    println!("Part one: {sum}");
    Ok(())
}

fn part_two(input_str: &String) -> Result<(), String> {
    let re_start = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_end = Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let input: Vec<u32> = input_str
        .split("\n")
        .map(|s|
            {
                let first_digit: u32 = parse_digit(
                    re_start
                        .find(s)
                        .unwrap()
                        .as_str()
                );
                let last_digit: u32 = parse_digit(
                    re_end
                        .find({
                            s.chars().rev().collect::<String>().as_str()
                        })
                        .unwrap()
                        .as_str()
                        .chars()
                        .rev()
                        .collect::<String>()
                        .as_str()
                );

                (first_digit * 10) + last_digit
            })
        .collect();

    let sum = input.iter().fold(0, |total, x| total + x);
    println!("Part two: {sum}");
    Ok(())
}

fn parse_digit(digit: &str) -> u32 {
    match digit.parse() {
        Ok(n) => n,
        Err(_) => match digit {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 999999,
        }
    }
}
