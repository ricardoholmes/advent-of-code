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
