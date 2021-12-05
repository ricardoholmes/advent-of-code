use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input_3.txt")
        .expect("Failed to read file");

    let input: Vec<&str> = input
        .split("\n")
        .collect();
    
    part_one(&input);
    part_two(&input);
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

fn part_two(input: &Vec<&str>) {
    let mut most_common = input.clone();
    let mut least_common = input.clone();

    for bit in 0..input[0].len() {
        if most_common.len() > 1 {
            let mut count = 0;

            for i in &most_common {
                if i.as_bytes()[bit] == '1' as u8 {
                    count += 1;
                }
            }

            let most_common_bit = (if count as f32 >= (most_common.len() as f32) / 2.0 { '1' } else { '0' }) as u8;
            most_common.retain(|&i| i.as_bytes()[bit] == most_common_bit);
        }

        if least_common.len() > 1 {
            let mut count = 0;

            for i in &least_common {
                if i.as_bytes()[bit] == '1' as u8 {
                    count += 1;
                }
            }

            let most_common_bit = (if count as f32 >= (least_common.len() as f32) / 2.0 { '1' } else { '0' }) as u8;
            least_common.retain(|&i| i.as_bytes()[bit] != most_common_bit);
        }
    }

    let o2_generator_rating: i32 = i32::from_str_radix(&most_common[0], 2).unwrap();
    let co2_scrubber_rating: i32 = i32::from_str_radix(&least_common[0], 2).unwrap();

    let life_support_rating = o2_generator_rating * co2_scrubber_rating;
    println!("Part 2: {}", life_support_rating);
}
