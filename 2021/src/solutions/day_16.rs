use std::fs;

pub fn run() {
    let input_str: String = fs::read_to_string("inputs/input_16.txt")
        .expect("Failed to read file");

    let mut input: String = String::new();

    for i in input_str.chars() {
        input += hex_to_bin(i);
    }

    let packet_info: (u32, u64, usize) = resolve_packet(&input, &0);
    println!("Part one: {}", packet_info.0);
    println!("Part two: {}", packet_info.1);
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

fn resolve_packet(input: &String, pointer: &usize) -> (u32, u64, usize) {
    let mut version_sum: u32 = 0;
    
    let mut pointer: usize = pointer.clone();

    let packet_version: u8 = u8::from_str_radix(&input[pointer..pointer+3], 2).unwrap();
    version_sum += packet_version as u32;
    pointer += 3;
    
    let packet_type: u8 = u8::from_str_radix(&input[pointer..pointer+3], 2).unwrap();
    pointer += 3;

    let packet_value: u64;

    if packet_type == 4 {
        let mut temp_packet_val: String = String::new();
        loop {
            pointer += 1;
            temp_packet_val += &input[pointer..pointer+4];
            pointer += 4;
            if input.chars().nth(pointer - 5).unwrap() == '0' {
                break;
            }
        }
        packet_value = u64::from_str_radix(&temp_packet_val, 2).unwrap();
    }

    else {
        let length_type_id: char = input.chars().nth(pointer).unwrap();
        pointer += 1;

        let mut packet_values: Vec<u64> = vec!();
        if length_type_id == '0' {
            let num_of_bits: usize = usize::from_str_radix(&input[pointer..pointer+15], 2).unwrap();
            pointer += 15;

            let mut temp_pointer: usize = pointer;
            while temp_pointer - pointer < num_of_bits {
                let packet_info: (u32, u64, usize) = resolve_packet(input, &temp_pointer);
                version_sum += packet_info.0;
                packet_values.push(packet_info.1);
                temp_pointer = packet_info.2;
            }
            pointer = temp_pointer;
        }

        else {
            let subpacket_count: usize = usize::from_str_radix(&input[pointer..pointer+11], 2).unwrap();
            pointer += 11;

            for _ in 0..subpacket_count {
                let packet_info: (u32, u64, usize) = resolve_packet(input, &pointer);
                version_sum += packet_info.0;
                packet_values.push(packet_info.1);
                pointer = packet_info.2;
            }
        }

        packet_value = match packet_type {
            0 => packet_values.iter().sum(),
            1 => packet_values.iter().product(),
            2 => *packet_values.iter().min().unwrap(),
            3 => *packet_values.iter().max().unwrap(),
            5 => if packet_values[0] > packet_values[1] { 1 } else { 0 },
            6 => if packet_values[0] < packet_values[1] { 1 } else { 0 },
            7 => if packet_values[0] == packet_values[1] { 1 } else { 0 },
            _ => panic!(), // won't happen
        };
    }

    (version_sum, packet_value, pointer)
}
