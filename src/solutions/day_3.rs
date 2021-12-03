use std::fs;

pub fn run() {

    let input = fs::read_to_string("inputs/input_3.txt")
        .expect("Failed to read file");

    let input: Vec<&str> = input
        .split("\n")
        .collect();
    
    part_one(&input);

}

fn part_one(input: &Vec<&str>) {

    let num_of_digits = input[0].len();
    let mut ones_count: Vec<i32> = vec![0; num_of_digits];

    for bit in 0..num_of_digits {

        for i in input {

            if i.as_bytes()[bit] == '1' as u8 {

                ones_count[bit] += 1;

            }

        }

    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for i in ones_count {

        if i >= (input.len() / 2) as i32 {

            gamma_rate += "1";
            epsilon_rate += "0";

        }

        else {

            gamma_rate += "0";
            epsilon_rate += "1";

        }

    }

    let gamma_rate: i32 = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate: i32 = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    let power_consumption = gamma_rate * epsilon_rate;
    println!("Part 1: {}", power_consumption);

}
