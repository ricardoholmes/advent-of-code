pub fn run() {
    let input_str = include_str!("../../inputs/input_3.txt");

    let input: Vec<(Vec<char>, Vec<char>)> = input_str
        .lines()
        .map(|i| (
            i.chars().take(i.len()/2).collect::<Vec<char>>(),
            i.chars().skip(i.len()/2).take(i.len()/2).collect::<Vec<char>>()
        ))
        .collect();

    part_one(&input);
}

fn part_one(input: &Vec<(Vec<char>, Vec<char>)>) {
    let mut total: i32 = 0;
    for i in input {
        let common_chars: Vec<&char> = i.0.iter().filter(|c| i.1.contains(c)).collect();

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
