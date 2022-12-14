use std::collections::HashSet;

pub fn run() {
    let input_str = include_str!("../../inputs/input_14.txt");

    let input: Vec<Vec<(u32, u32)>> = input_str
        .lines()
        .map(|line| line.split(" -> ")
            .map(|coord| {
                let nums: Vec<u32> = coord
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();

                (nums[0], nums[1])
            }).collect::<Vec<(u32, u32)>>()
        ).collect();

    let mut points: HashSet<(u32, u32)> = HashSet::new();
    for line in input {
        for i in 0..line.len()-1 {
            if line[i+1].0 > line[i].0 {
                for x in line[i].0..=line[i+1].0 {
                    points.insert((x, line[i].1));
                }
            }
            else if line[i+1].0 < line[i].0 {
                for x in line[i+1].0..=line[i].0 {
                    points.insert((x, line[i].1));
                }
            }
            else if line[i+1].1 > line[i].1 {
                for y in line[i].1..=line[i+1].1 {
                    points.insert((line[i].0, y));
                }
            }
            else if line[i+1].1 < line[i].1 {
                for y in line[i+1].1..=line[i].1 {
                    points.insert((line[i].0, y));
                }
            }
        }
    }

    part_one(&points);
    part_two(&points);
}

fn part_one(points: &HashSet<(u32, u32)>) {
    let mut points = points.clone();
    let sand_spawn = (500, 0);
    
    let lowest_y = points.iter().map(|point| point.1).max().unwrap();

    let mut sand_pos = sand_spawn;
    let mut unit_count = 0;
    loop {
        if !points.contains(&(sand_pos.0, sand_pos.1+1)) {
            sand_pos.1 += 1;
        }
        else if !points.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos.1 += 1; 
            sand_pos.0 -= 1; 
        }
        else if !points.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos.1 += 1; 
            sand_pos.0 += 1; 
        }
        else {
            unit_count += 1;
            points.insert(sand_pos);
            if sand_pos == sand_spawn {
                break;
            }
            sand_pos = sand_spawn;
        }

        if sand_pos.1 > lowest_y {
            break;
        }
    }

    println!("Part one: {unit_count}");
}

fn part_two(points: &HashSet<(u32, u32)>) {
    let mut points = points.clone();
    let sand_spawn = (500, 0);
    
    let lowest_y = points.iter().map(|point| point.1).max().unwrap() + 1;

    let mut sand_pos = sand_spawn;
    let mut unit_count = 0;
    loop {
        if !points.contains(&(sand_pos.0, sand_pos.1+1)) {
            sand_pos.1 += 1;
        }
        else if !points.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos.1 += 1; 
            sand_pos.0 -= 1; 
        }
        else if !points.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos.1 += 1; 
            sand_pos.0 += 1; 
        }
        else {
            unit_count += 1;
            points.insert(sand_pos);
            if sand_pos == sand_spawn {
                break;
            }
            sand_pos = sand_spawn;
        }

        if sand_pos.1 == lowest_y {
            unit_count += 1;
            points.insert(sand_pos);
            sand_pos = sand_spawn;
        }
    }

    println!("Part Two: {unit_count}");
}
