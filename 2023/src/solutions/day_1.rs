pub fn run() -> Result<(), String> {
    let input_str = include_str!("../../inputs/input_1.txt");

    let input_str = input_str
        .strip_suffix('\n')
        .unwrap_or(input_str)
        .to_string();

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

    match part_one(&input) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };
    part_two(&input)
}

fn part_one(input: &[u32]) -> Result<(), String> {
    let sum = input.iter().fold(0, |total, x| total + x);
    println!("Part one: {sum}");
    Ok(())
}

fn part_two(input: &[u32]) -> Result<(), String> {
    println!("Part two: HELLO");
    Ok(())
}
