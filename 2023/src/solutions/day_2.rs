use crate::safe_unpack;

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

pub fn run() -> Result<(), String> {
    let input: Vec<&str> = include_str!("../../inputs/input_2.txt").lines().collect();

    // each element is: (id, [max_red_count, max_green_count, max_blue_count])
    let mut games: Vec<Game> = vec![];

    for line in &input {
        safe_unpack!(line.split(": "), id, game_str);

        let mut game = Game {
            id: id.split_ascii_whitespace().last().unwrap().parse().unwrap(),
            red: 0,
            green: 0,
            blue: 0,
        };

        let game_str = game_str.replace("; ", ", ");
        let game_properties = game_str.split(", ");

        for property in game_properties {
            safe_unpack!(property.split_ascii_whitespace(), count, color);

            let count: u32 = match count.parse() {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };

            if color == "red" && count > game.red {
                game.red = count;
            }
            if color == "green" && count > game.green {
                game.green = count;
            }
            if color == "blue" && count > game.blue {
                game.blue = count;
            }
        }

        games.push(game);
    }

    match part_one(&games) {
        Ok(n) => println!("Part one: {n}"),
        Err(e) => return Err(e),
    };

    match part_two(&games) {
        Ok(n) => println!("Part two: {n}"),
        Err(e) => return Err(e),
    };

    Ok(())
}

fn part_one(games: &[Game]) -> Result<u32, String> {
    Ok(games.iter().fold(0, |total, game| {
        total
            + if game.red > 12 || game.green > 13 || game.blue > 14 {
                0
            } else {
                game.id
            }
    }))
}

fn part_two(games: &[Game]) -> Result<u32, String> {
    Ok(games
        .iter()
        .map(|game| game.red * game.green * game.blue)
        .sum())
}
