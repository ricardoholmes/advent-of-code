pub fn run() {
    let input: Vec<&str> = include_str!("../../inputs/input_3.txt")
        .lines()
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<&str>) {
    // split input in two
    let input: Vec<(Vec<char>, Vec<char>)> = input
        .iter()
        .map(|i| (
            i.chars().take(i.len()/2).collect::<Vec<char>>(),
            i.chars().skip(i.len()/2).collect::<Vec<char>>()
        ))
        .collect();

    let mut total: i32 = 0;
    for i in input {
        let common_chars: Vec<&char> = i.0
            .iter()
            .filter(|c| i.1.contains(c))
            .collect();

        let common = common_chars[0];

        if common.is_lowercase() {
            total += (*common as i32) - 96;
        }
        else {
            total += (*common as i32) - 64 + 26;
        }
    }

    println!("Part one: {total}");
}

fn part_two(input: &Vec<&str>) {
    let mut total: i32 = 0;
    for i in (0..input.len()).step_by(3) {
        let common_chars: Vec<char> = input[i]
            .chars()
            .filter(|c| input[i+1].contains(*c) && input[i+2].contains(*c))
            .collect();

        let common: char = common_chars[0];

        if common.is_lowercase() {
            total += (common as i32) - 96;
        }
        else {
            total += (common as i32) - 64 + 26;
        }
    }

    println!("Part two: {total}");
}
