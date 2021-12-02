use std::fs;

pub fn run() {
    let input_str = fs::read_to_string("inputs/input_2.txt")
        .expect("Failed to read file");

    let input: Vec<&str> = input_str
        .split("\n")
        .collect();
    
        part1(input)
}

fn part1(input: Vec<&str>) {
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