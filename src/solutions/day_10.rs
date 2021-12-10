use std::fs;

pub fn run() {
    // read the file and store it in input
    let input = fs::read_to_string("inputs/input_10.txt")
        .expect("Failed to read file");

    // split the lines of the file
    let input: Vec<&str> = input
        .split("\n")
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(lines: &Vec<&str>) {
    // constants for open and close brackets
    let open_brackets = ['(', '[', '{', '<'];
    let close_brackets = [')', ']', '}', '>'];

    // total syntax error score
    let mut syntax_error_score: u32 = 0;
    for line in lines {
        let mut brackets_needed: Vec<char> = vec!();
        // for each bracket on the line
        for i in line.chars() {
            // if it's an open bracket
            if open_brackets.contains(&i) {
                // add the corresponding close bracket to the start of the list of needed brackets
                let bracket_index: usize = open_brackets.iter().position(|&r| r == i).unwrap();
                brackets_needed.insert(0, close_brackets[bracket_index]);
            }

            // if it's a close bracket
            else {
                // if the bracket is valid
                if brackets_needed.len() > 0 && i == brackets_needed[0] {
                    // remove it from the start of the needed brackets
                    brackets_needed.remove(0);
                }

                // if the bracket is invalid
                else {
                    // add the respective syntax score to the total syntax score, and exit the loop
                    syntax_error_score += match i {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    };
                    break;
                }
            }
        }
    }

    // output total syntax error score
    println!("Part 1: {}", syntax_error_score);
}

fn part_two(lines: &Vec<&str>) {
    let open_brackets = ['(', '[', '{', '<'];
    let close_brackets = [')', ']', '}', '>'];
    let mut all_scores: Vec<u64> = vec!();
    for line in lines {
        let mut brackets_needed: Vec<char> = vec!();
        let mut is_corrupt = false;
        for i in line.chars() {
            // if it's an open bracket
            if open_brackets.contains(&i) {
                let bracket_index: usize = open_brackets.iter().position(|&r| r == i).unwrap();
                brackets_needed.insert(0, close_brackets[bracket_index]);
            }

            // if it's a close bracket
            else {
                if brackets_needed.len() > 0 && i == brackets_needed[0] {
                    brackets_needed.remove(0);
                }

                else {
                    is_corrupt = true;
                    break;
                }
            }
        }

        if !is_corrupt {
            let mut score: u64 = 0;
            for i in brackets_needed {
                score *= 5;
                score += match i {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                };
            }
            all_scores.push(score);
        }
    }

    // sort the vector
    all_scores.sort();
    // ouput the median
    println!("Part 2: {}", all_scores[all_scores.len() / 2]);
}
