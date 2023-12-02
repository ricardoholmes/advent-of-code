pub fn run() -> Result<(), String> {
    let input_str: Vec<&str> = include_str!("../../inputs/input_2.txt")
        .lines()
        .collect();

    match part_one(&input_str) {
        Ok(n) => println!("Part one: {n}"),
        Err(e) => return Err(e),
    };

    match part_two(&input_str) {
        Ok(n) => println!("Part two: {n}"),
        Err(e) => return Err(e),
    };

    Ok(())
}

fn part_one(lines: &[&str]) -> Result<u32, String> {
    let mut total: u32 = 0;
    for &i in lines {
        let line_split: Vec<&str> = i.split(": ").collect();
        let id: u32 = line_split[0].split_ascii_whitespace().last().unwrap().parse().unwrap();
        let game = line_split[1].replace("; ", ", ");
        let game_properties = game.split(", ");

        let mut valid = true;
        for property in game_properties {
            let split: Vec<&str> = property.split_ascii_whitespace().collect();
            
            let count: u32 = split[0].parse().unwrap();
            let color = split[1];

            if (color == "red" && count > 12) ||
                (color == "green" && count > 13) ||
                (color == "blue" && count > 14) {
                    valid = false;
                    break;
                }
        }
        if valid {
            total += id;
        }
    }
    Ok(total)
}

fn part_two(lines: &[&str]) -> Result<u32, String> {
    let mut power: u32 = 0;
    for &i in lines {
        let line_split: Vec<&str> = i.split(": ").collect();
        let game = line_split[1].replace("; ", ", ");
        let game_properties = game.split(", ");

        let mut red_count = 0;
        let mut blue_count = 0;
        let mut green_count = 0;
        for property in game_properties {
            let split: Vec<&str> = property.split_ascii_whitespace().collect();
            
            let count: u32 = split[0].parse().unwrap();
            let color = split[1];

            if color == "red" && count > red_count {
                red_count = count;
            }
            if color == "blue" && count > blue_count {
                blue_count = count;
            }
            if color == "green" && count > green_count {
                green_count = count;
            }
        }

        power += red_count * blue_count * green_count;
    }
    Ok(power)
}
