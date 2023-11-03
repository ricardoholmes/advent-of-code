pub fn run() {
    let input_str = include_str!("../../inputs/input_1.txt")
        .strip_suffix('\n')
        .unwrap();

    let input: Vec<u32> = input_str
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();

    part_one(&input);
    // part_two(&input);
}

fn part_one(input: &[u32]) {
    // get the biggest sum
    println!("Part one: HELLO");
}

fn part_two(input: &[u32]) {
    println!("Part two: HELLO");
}
