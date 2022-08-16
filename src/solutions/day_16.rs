use std::fs;

pub fn run() {
    let input_str: String = fs::read_to_string("inputs/input_16.txt")
        .expect("Failed to read file");

    let mut input: String = String::new();

    for i in input_str.chars() {
        input += hex_to_bin(i);
    }

    part_one(input);
}

fn hex_to_bin(hex: char) -> &'static str {
    match hex {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Non-hex digit ({}) found in input file", hex),
    }
}

fn get_version_sum(input: &String, pointer: &usize) -> (u32, usize) {
    let mut version_sum: u32 = 0;
    
    let mut pointer: usize = pointer.clone();

    let packet_version: u8 = u8::from_str_radix(&input[pointer..pointer+3], 2).unwrap();
    version_sum += packet_version as u32;
    pointer += 3;
    
    let packet_type: u8 = u8::from_str_radix(&input[pointer..pointer+3], 2).unwrap();
    pointer += 3;

    if packet_type == 4 {
        while input.chars().nth(pointer).unwrap() == '1' {
            // getting value of subpacket is unnecessary for part 1 so no need doing it (for now)
            pointer += 5;
        }
        pointer += 5;
    }

    else {
        let length_type_id: char = input.chars().nth(pointer).unwrap();
        pointer += 1;

        if length_type_id == '0' {
            let num_of_bits: usize = usize::from_str_radix(&input[pointer..pointer+15], 2).unwrap();
            pointer += 15;
            let mut temp_pointer: usize = pointer;
            while temp_pointer - pointer < num_of_bits - 1 {
                let packet_info: (u32, usize) = get_version_sum(input, &temp_pointer);
                version_sum += packet_info.0;
                temp_pointer = packet_info.1;
            }
            pointer = temp_pointer;
        }

        else {
            let subpacket_count: usize = usize::from_str_radix(&input[pointer..pointer+11], 2).unwrap();
            pointer += 11;
            for _ in 0..subpacket_count {
                let packet_info: (u32, usize) = get_version_sum(input, &pointer);
                version_sum += packet_info.0;
                pointer = packet_info.1;
            }
        }
    }

    (version_sum, pointer)
}

fn part_one(input: String) {
    let version_sum: u32 = get_version_sum(&input, &0).0;
    println!("Part one: {}", version_sum);
}
