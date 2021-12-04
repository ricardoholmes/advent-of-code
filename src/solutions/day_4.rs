use std::fs;

pub fn run() {

    let input_str = fs::read_to_string("inputs/input_4.txt")
        .expect("Failed to read file")
        .replace("  ", " ")
        .replace("\n ", "\n");

    let input: Vec<&str> = input_str
        .split("\n\n")
        .collect();

    let numbers: Vec<&str> = input[0]
        .split(",")
        .collect();

    let boards_strings: Vec<&str> = input[1..]
        .to_vec();

    let mut boards: Vec<Vec<Vec<&str>>> = vec!();
    for i in &boards_strings {

        let mut board: Vec<Vec<&str>> = vec!();
        for j in i.split("\n") {

            board.push(j.split(" ").collect());

        }

        boards.push(board);

    }

    part_one(&numbers, &boards, &boards_strings);
    part_two(&numbers, &boards, &boards_strings);

}

fn part_one(numbers: &Vec<&str>, boards: &Vec<Vec<Vec<&str>>>, boards_strings: &Vec<&str>) {

    let mut numbers_picked: Vec<&str> = vec!();
    for i in numbers {

        numbers_picked.push(i);

        for j in 0..boards.len() {

            if boards_strings[j].contains(i) {

                let (y, x) = position_of(&boards[j], &i);

                // check row
                let mut bingo: bool = true;
                for value in &boards[j][y] {

                    if !numbers_picked.contains(value) {

                        bingo = false;
                        break;

                    }

                }

                if !bingo {

                    // check column
                    bingo = true;
                    for row in &boards[j] {

                        if !numbers_picked.contains(&row[x]) {

                            bingo = false;
                            break;

                        }

                    }

                }

                if bingo {

                    let score = score_of_board(&boards[j], &numbers_picked) * i.parse::<i32>().expect("Error parsing");
                    println!("Part 1: {}", score);
                    return ();

                }

            }

        }

    }

}

fn part_two(numbers: &Vec<&str>, boards: &Vec<Vec<Vec<&str>>>, boards_strings: &Vec<&str>) {

    let mut boards_won: Vec<usize> = vec!();
    let mut numbers_picked: Vec<&str> = vec!();
    for i in numbers {

        numbers_picked.push(i);

        for j in 0..boards.len() {

            if boards_strings[j].contains(i) {

                let (y, x) = position_of(&boards[j], &i);

                // check row
                let mut bingo: bool = true;
                for value in &boards[j][y] {

                    if !numbers_picked.contains(value) {

                        bingo = false;
                        break;

                    }

                }

                if !bingo {

                    // check column
                    bingo = true;
                    for row in &boards[j] {

                        if !numbers_picked.contains(&row[x]) {

                            bingo = false;
                            break;

                        }

                    }

                }

                if bingo && !boards_won.contains(&j) {

                    if boards.len() - 1 > boards_won.len() {

                        boards_won.push(j);

                    }

                    else {

                        let score = score_of_board(&boards[j], &numbers_picked) * i.parse::<i32>().expect("Error parsing");
                        println!("Part 2: {}", score);
                        return ();

                    }

                }

            }

        }

    }

}

fn position_of(grid: &Vec<Vec<&str>>, value: &str) -> (usize, usize) {

    for y in 0..grid.len() {

        for x in 0..grid[y].len() {

            if grid[y][x] == value {

                return (y, x);

            }

        }

    }

    (0, 0)

}

fn score_of_board(board: &Vec<Vec<&str>>, values_picked: &Vec<&str>) -> i32 {

    let mut score: i32 = 0;
    for i in board {

        for j in i {

            if !values_picked.contains(j) {

                score += j.parse::<i32>()
                    .expect("Failed parse");

            }

        }

    }

    score

}
