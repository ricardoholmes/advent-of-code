use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input_8.txt")
        .expect("Failed to read file");

    let input: Vec<&str> = input
        .split("\n")
        .collect();
    
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<&str>) {
    let mut count = 0;
    for line in input {
        let output: Vec<&str> = line
            .split(" | ")
            .collect();

        let output_split: Vec<&str> = output[1]
            .split(" ")
            .collect();

        for i in output_split {
            let length = i.chars().count();
            if length == 2 || length == 3 || length == 4 || length == 7 {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}

fn part_two(input: &Vec<&str>) {
    let mut total: i32 = 0;
    for line in input {
        let output_digits: Vec<&str> = line
            .split(" | ")
            .collect::<Vec<&str>>()[1]
            .split(" ")
            .collect();

        let digits: &str = &line
            .replace(" | ", " ");

        let digits: Vec<&str> = digits
            .split(" ")
            .collect();

        let mut output: [char; 4] = [' '; 4];

        for i in 0..4 {
            output[i] = match output_digits[i].len() {
                2 => '1',
                3 => '7',
                4 => '4',
                5 => {
                    let mut digit = '2';
                    for j in &digits {
                        // if it contains 7
                        if j.len() == 3 && contains(output_digits[i], j) {
                            digit = '3';
                            break;
                        }

                        else if j.len() == 6 {
                            for k in &digits {
                                // if it contains a 4, it means it's a 9
                                if k.len() == 4 && contains(j, k) {
                                    // if 9 contains the output digit,
                                    // the output digit is either 3 or 5
                                    // so don't fully break
                                    if contains(j, output_digits[i]) {
                                        digit = '5';
                                    }

                                    break;
                                }
                            }
                        }
                    }
                    digit
                },
                6 => {
                    let mut digit = '0';
                    for j in &digits {
                        if j.len() == 3 && !contains(output_digits[i], j) {
                            digit = '6';
                            break;
                        }

                        else if j.len() == 4 && contains(output_digits[i], j) {
                            digit = '9';
                            break;
                        }
                    }
                    digit
                },
                7 => '8',
                _ => {
                    println!("Invalid digit found ({})", output_digits[i]);
                    ' '
                },
            };
        }

        total += output
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .expect("Parsing error");
    }

    println!("Part 2: {}", total);
}

// string contains substring
fn contains(string: &str, substring: &str) -> bool {
    for i in substring.chars() {
        if !string.contains(i) {
            return false;
        }
    }
    true
}
