pub fn run() {
    let input_str = include_str!("../../inputs/input_2.txt");

    // split input into a list of lists (size 2) of &strs
    // each internal list is a round, eg. ["A", "X"] 
    let input: Vec<Vec<&str>> = input_str
        .lines()
        .map(|i| i
            .split(" ")
            .collect::<Vec<&str>>()
        ).collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<Vec<&str>>) {
    let mut score = 0;
    for game in input {
        // increase score by result points (6 for win, 3 for draw, 0 for loss)
        score += match game[0] {
            "A" => if game[1] == "Y" { 6 } else { if game[1] == "X" { 3 } else { 0 } },
            "B" => if game[1] == "Z" { 6 } else { if game[1] == "Y" { 3 } else { 0 } },
            "C" => if game[1] == "X" { 6 } else { if game[1] == "Z" { 3 } else { 0 } },
            _ => {
                println!("Invalid character '{}'", game[0]);
                0
            },
        };

        // increase score by choice points (1 for X, 2 for Y, 3 for Z)
        score += match game[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => {
                println!("Invalid character '{}'", game[1]);
                0
            },
        };
    }

    println!("Part one: {score}");
}

fn part_two(input: &Vec<Vec<&str>>) {
    let mut score = 0;
    for game in input {
        score += match game[1] {
            "X" => 0 + if game[0] == "A" { 3 } else { if game[0] == "C" { 2 } else { 1 }},
            "Y" => 3 + if game[0] == "C" { 3 } else { if game[0] == "B" { 2 } else { 1 }},
            "Z" => 6 + if game[0] == "B" { 3 } else { if game[0] == "A" { 2 } else { 1 }},
            _ => {
                println!("Invalid character '{}'", game[0]);
                0
            },
        };
    }

    println!("Part two: {score}");
}
