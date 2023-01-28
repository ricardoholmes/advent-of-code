pub fn run() {
    let input_str = include_str!("../../inputs/input_4.txt")
        .replace("\r\n", "\n");

    let input: Vec<Vec<i32>> = input_str
        .lines()
        .map(|pair| pair
            .split(&['-', ','])
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i32>>()
        ).collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<Vec<i32>>) {
    let mut count = 0;
    for line in input {
        if (line[0] <= line[2] && line[1] >= line[3]) ||
            (line[0] >= line[2] && line[1] <= line[3]) {
            count += 1;
        }
    }

    println!("Part one: {count}");
}

fn part_two(input: &Vec<Vec<i32>>) {
    let mut count = 0;
    for line in input {
        if (line[0] <= line[3] && line[1] >= line[3]) ||
            (line[0] <= line[2] && line[1] >= line[2]) ||
            (line[0] >= line[2] && line[1] <= line[3]) {
            count += 1;
        }
    }

    println!("Part two: {count}");
}
