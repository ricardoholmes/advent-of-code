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
    let input: Vec<u32> = input_str
        .lines()
        .map(|s|
            {
                let digits = s.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();

                let num: String = format!("{}{}", digits.chars().nth(0).unwrap(), digits.chars().nth(digits.len()-1).unwrap());

                num.parse().expect("Failed to parse")
            })
        .collect();

    Ok(input.iter().fold(0, |total, x| total + x))
}

fn part_two(input_str: &&str) -> Result<u32, String> {
    let digit_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let input: Vec<u32> = input_str
        .lines()
        .map(|s| {
            let digits = (0..s.len()).filter_map(|i|
                match s.chars().nth(i).unwrap().to_digit(10) {
                    Some(n) => Some(n),
                    None => match digit_words
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
            );

            (10 * digits.clone().next().unwrap()) + digits.clone().last().unwrap()
        })
        .collect();

    Ok(input.iter().fold(0, |total, x| total + x))
}
