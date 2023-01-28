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
    let mut cannot_contain_ranges: Vec<i64> = vec![];
    let mut x_beacons_on_row: HashSet<i64> = HashSet::new();

    for sensor in sensors {
        let distance = sensor.beacon_rel_pos.0.abs() + sensor.beacon_rel_pos.1.abs();
        if sensor.pos.1 + distance < ROW || sensor.pos.1 - distance > ROW {
            continue;
        }
        let max_x_offset: i64 = if sensor.pos.1 < ROW { sensor.pos.1 + distance - ROW } else { ROW - (sensor.pos.1 - distance) };

        if sensor.beacon_rel_pos.1 == ROW {
            x_beacons_on_row.insert(sensor.beacon_rel_pos.0);
        }

        let min_x = sensor.pos.0 - max_x_offset;
        let max_x = sensor.pos.0 + max_x_offset;
        insert_range(&mut cannot_contain_ranges, &(min_x, max_x));
    }

    let mut ranges: Vec<i64> = vec![];
    for range in cannot_contain_ranges.chunks(2) {
        insert_range(&mut ranges, &(range[0], range[1]));
    }

    let mut count = 0;
    for range in ranges.chunks(2) {
        count += range[1] - range[0];
        for &beacon in &x_beacons_on_row {
            if beacon >= range[0] && beacon <= range[1] {
                count -= 1;
            }
        }
    }
    println!("Part one: {}", count);
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

fn insert_range(ranges: &mut Vec<i64>, range_to_insert: &(i64, i64)) {
    let mut inserted = false;
    let min = range_to_insert.0;
    let max = range_to_insert.1;
    for i in 0..ranges.len()/2 {
        let range = &mut ranges[i*2..=(i*2)+1];
        if min >= range[0] && min <= range[1] && max > range[1] {
            range[1] = max;
            inserted = true;
            break;
        }
        else if min < range[0] && max >= range[0] && max <= range[1] {
            range[0] = min;
            inserted = true;
            break;
        }
        else if min < range[0] && max > range[1] {
            range[0] = min;
            range[1] = max;
            inserted = true;
            break;
        }
        else if min >= range[0] && max <= range[1] {
            inserted = true;
            break;
        }
        else {
        }
    }
    
    if !inserted {
        ranges.push(min);
        ranges.push(max);
    }
}
