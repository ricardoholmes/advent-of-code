pub fn run() {
    let input_str = include_str!("../../inputs/input_1.txt");

    // make input into list of sums of calories
    let mut input: Vec<u32> = input_str
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|s| s.split("\n")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .sum()
        )
        .collect();

    // sort in decending order
    input.sort();
    input.reverse();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<u32>) {
    // get the biggest sum
    println!("Part one: {}", input[0]);
}

fn part_two(input: &Vec<u32>) {
    // add the top 3 biggest sums
    let total: u32 = input.iter().take(3).sum();
    println!("Part two: {total}");
}
