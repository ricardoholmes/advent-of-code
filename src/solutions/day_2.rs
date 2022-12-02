pub fn run() {
    let input_str = include_str!("../../inputs/input_2.txt");

    let input: Vec<&str> = input_str
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .collect();

    part_one(&input);
}

fn part_one(input: &Vec<&str>) {
    let games: Vec<Vec<&str>> = input
        .iter()
        .map(|i| i
            .split(" ")
            .collect::<Vec<&str>>()
        ).collect();

    let mut score = 0;
    for game in games {
        score += match game[0] {
            "A" => if game[1] == "Y" { 6 } else { if game[1] == "X" { 3 } else { 0 } },
            "B" => if game[1] == "Z" { 6 } else { if game[1] == "Y" { 3 } else { 0 } },
            "C" => if game[1] == "X" { 6 } else { if game[1] == "Z" { 3 } else { 0 } },
            _ => {
                println!("Invalid character '{}'", game[0]);
                0
            },
        };

        score += match game[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => {
                println!("Invalid character '{}'", game[1]);
                0
            },
        }
    }

    println!("Part one: {score}")
}
