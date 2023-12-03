pub fn run(input_raw: &str) -> Result<(), String> {
    let answer_part_one = part_one(&input_raw)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&input_raw)?;
    println!("Part two: {}", answer_part_two);

    Ok(())
}

fn part_one(input_str: &&str) -> Result<u32, String> {
    match parse_input(input_str, true) {
        Ok(input) => Ok(input.iter().sum()),
        Err(e) => Err(e),
    }
}

fn part_two(input_str: &&str) -> Result<u32, String> {
    match parse_input(input_str, false) {
        Ok(input) => Ok(input.iter().sum()),
        Err(e) => Err(e),
    }
}

fn parse_input(input_str: &&str, only_digits: bool) -> Result<Vec<u32>, String> {
    const DIGIT_WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let parsed: Vec<u32> = input_str
        .lines()
        .map(|s| {
            let digits =
                (0..s.len()).filter_map(|i| match s.chars().nth(i).unwrap().to_digit(10) {
                    Some(n) => Some(n),
                    None => {
                        if only_digits {
                            None
                        } else {
                            match DIGIT_WORDS
                                .iter()
                                .find(|&&d| s.len() - i >= d.len() && &s[i..d.len() + i] == d)
                            {
                                Some(&"one") => Some(1),
                                Some(&"two") => Some(2),
                                Some(&"three") => Some(3),
                                Some(&"four") => Some(4),
                                Some(&"five") => Some(5),
                                Some(&"six") => Some(6),
                                Some(&"seven") => Some(7),
                                Some(&"eight") => Some(8),
                                Some(&"nine") => Some(9),
                                _ => None,
                            }
                        }
                    }
                });

            (10 * digits.clone().next().unwrap()) + digits.last().unwrap()
        })
        .collect();

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example: &str = include_str!("../../examples/day_1_1.txt");
        let result = part_one(&example);
        assert_eq!(result, Ok(142));
    }

    #[test]
    fn test_part2() {
        let example: &str = include_str!("../../examples/day_1_2.txt");
        let result = part_two(&example);
        assert_eq!(result, Ok(281));
    }
}
