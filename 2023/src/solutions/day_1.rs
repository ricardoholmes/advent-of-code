pub fn run() -> Result<(), String> {
    let input_str = include_str!("../../inputs/input_1.txt");

    let input_str = input_str
        .strip_suffix('\n')
        .unwrap_or(input_str)
        .to_string();

    let input: Vec<u32> = input_str
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();

    match part_one(&input) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };
    part_two(&input)
}

fn part_one(input: &[u32]) -> Result<(), String> {
    println!("Part one: HELLO");
    Ok(())
}

fn part_two(input: &[u32]) -> Result<(), String> {
    println!("Part two: HELLO");
    Ok(())
}
