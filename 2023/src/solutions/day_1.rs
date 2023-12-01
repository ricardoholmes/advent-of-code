pub fn run() -> Result<(), String> {
    let input_str = include_str!("../../inputs/input_1.txt");

    match part_one(&input_str) {
        Ok(n) => println!("Part one: {n}"),
        Err(e) => return Err(e),
    };

    match part_two(&input_str) {
        Ok(n) => println!("Part one: {n}"),
        Err(e) => return Err(e),
    };

    Ok(())
}

fn part_one(input_str: &&str) -> Result<u32, String> {
    match parse_input(input_str, false) {
        Ok(input) => Ok(input.iter().sum()),
        Err(e) => Err(e),
    }
}

fn part_two(input_str: &&str) -> Result<u32, String> {
    match parse_input(input_str, true) {
        Ok(input) => Ok(input.iter().sum()),
        Err(e) => Err(e),
    }
}

fn parse_input(input_str: &&str, only_digits: bool) -> Result<Vec<u32>, String> {
    let digit_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    Ok(input_str
        .lines()
        .map(|s| {
            let digits = (0..s.len()).filter_map(|i|
                match s.chars().nth(i).unwrap().to_digit(10) {
                    Some(n) => Some(n),
                    None => if only_digits {
                            None 
                        }
                        else {
                            match digit_words
                                .iter()
                                .filter(|&&d| s.len() - i >= d.len() && &s[i..d.len()+i] == d)
                                .next() {
                                    Some(&"one") => Some(1),
                                    Some(&"two") => Some(2),
                                    Some(&"three") => Some(3),
                                    Some(&"four") => Some(4),
                                    Some(&"five") => Some(5),
                                    Some(&"six") => Some(6),
                                    Some(&"seven") => Some(7),
                                    Some(&"eight") => Some(8),
                                    Some(&"nine") => Some(9),
                                    Some(_) => None,
                                    None => None,
                                }
                        }
                }
            );

            (10 * digits.clone().next().unwrap()) + digits.last().unwrap()
        })
        .collect()
    )
}
