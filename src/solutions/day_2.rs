use std::fs;

pub fn run() {
    let input_str = fs::read_to_string("inputs/input_2.txt")
        .expect("Failed to read file");

    part1((*input_str).to_string());
    part2((*input_str).to_string());
}

fn part1(input_str: String) {
    let input: Vec<&str> = input_str
        .split("\n")
        .collect();

    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;
    for command in input {
        let command_split: Vec<&str> = command
            .split(" ")
            .collect();

        let direction = command_split[0];
        let magnitude: i32 = (*command_split[1])
            .parse()
            .expect("Error parsing magnitude");

        match direction {
            "forward" => horizontal_pos += magnitude,
            "up" => vertical_pos -= magnitude,
            "down" => vertical_pos += magnitude,
            _ => println!("Error with direction")
        }
    }

    println!("{}", (horizontal_pos * vertical_pos))
}

fn part2(input_str: String) {
    let input: Vec<&str> = input_str
        .split("\n")
        .collect();

    let mut aim = 0;
    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;
    for command in input {
        let command_split: Vec<&str> = command
            .split(" ")
            .collect();

        let direction = command_split[0];
        let magnitude: i32 = (*command_split[1])
            .parse()
            .expect("Error parsing magnitude");

        match direction {
            "forward" => {
                horizontal_pos += magnitude;
                vertical_pos += aim * magnitude;
            },
            "up" => aim -= magnitude,
            "down" => aim += magnitude,
            _ => println!("Error with direction")
        }
    }

    println!("{}", (horizontal_pos * vertical_pos))
}
