use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    beacon_rel_pos: (i64, i64),
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_15.txt");

    let input: Vec<Sensor> = input_str
        .lines()
        .map(|line| {
            let mut nums: Vec<i64> = vec![];
            let mut index = 0;
            for _ in 0..4 {
                let mut num_string: String = String::new();
                index = index + line[index..].chars().position(|c| c == '=').unwrap() + 1;
                while line.chars().nth(index).unwrap_or('\0').is_ascii_digit()
                    || line.chars().nth(index).unwrap_or('\0') == '-'
                    || line.chars().nth(index).unwrap_or('\0') == '+' {
                    num_string.push(line.chars().nth(index).unwrap());
                    index += 1;
                }
                nums.push(num_string.parse().unwrap());
            }

            Sensor {
                pos: (nums[0], nums[1]),
                beacon_rel_pos: (nums[2] - nums[0], nums[3] - nums[1]),
            }
        }
        ).collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(sensors: &Vec<Sensor>) {
    const ROW: i64 = 2_000_000;
    let mut cannot_contain: HashSet<(i64, i64)> = HashSet::new();

    for sensor in sensors {
        let distance = sensor.beacon_rel_pos.0.abs() + sensor.beacon_rel_pos.1.abs();
        if sensor.pos.1 + distance < ROW || sensor.pos.1 - distance > ROW {
            continue;
        }
        let max_x_offset: i64 = if sensor.pos.1 < ROW { sensor.pos.1 + distance - ROW } else { ROW - (sensor.pos.1 - distance) };

        for x_offset in -max_x_offset..=max_x_offset {
            if sensor.beacon_rel_pos != (x_offset, ROW - sensor.pos.1) {
                cannot_contain.insert((sensor.pos.0 + x_offset, ROW));
            }
        }
    }

    println!("Part one: {}", cannot_contain.len());
}

fn part_two(sensors: &Vec<Sensor>) {
    const MAX_XY: i64 = 4_000_000;

    for y in 0..=MAX_XY {
        let mut x = 0;
        while x <= MAX_XY {
            let mut detected: bool = false;
            for sensor in sensors {
                let distance = sensor.beacon_rel_pos.0.abs() + sensor.beacon_rel_pos.1.abs();
                if sensor.pos.1 + distance < y || sensor.pos.1 - distance > y {
                    continue;
                }
                let max_x_offset: i64 = if sensor.pos.1 < y { sensor.pos.1 + distance - y } else { y - (sensor.pos.1 - distance) };
                if sensor.pos.0 - max_x_offset <= x && sensor.pos.0 + max_x_offset >= x {
                    x = sensor.pos.0 + max_x_offset + 1;
                    detected = true;
                    break;
                }
            }
            if !detected {
                println!("Part two: {}", 4_000_000*x + y);
                return;
            }
        }
    }
}
