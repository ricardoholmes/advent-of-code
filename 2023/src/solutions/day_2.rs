use crate::safe_unpack;

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

pub fn run() -> Result<(), String> {
    let input: Vec<&str> = include_str!("../../inputs/input_2.txt").lines().collect();

    let games = match parse_input(&input) {
        Ok(games) => games,
        Err(e) => return Err(e),
    };

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

fn parse_input(lines: &[&str]) -> Result<Vec<Game>, String> {
    let mut games: Vec<Game> = vec![];

    for line in lines {
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

    Ok(games)
}

fn part_one(games: &[Game]) -> Result<u32, String> {
    let total = games.iter().fold(0, |total, game| {
        total
            + if game.red > 12 || game.green > 13 || game.blue > 14 {
                0
            } else {
                game.id
            }
    });

    Ok(total)
}

fn part_two(games: &[Game]) -> Result<u32, String> {
    let power_sum = games
        .iter()
        .map(|game| game.red * game.green * game.blue)
        .sum();

    Ok(power_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example: Vec<&str> = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .trim()
            .lines()
            .collect();

        let parsed = parse_input(&example).unwrap();
        let result = part_one(&parsed);
        assert_eq!(result, Ok(8));
    }

    #[test]
    fn test_part2() {
        let example: Vec<&str> = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .trim()
            .lines()
            .collect();

        let parsed = parse_input(&example).unwrap();
        let result = part_two(&parsed);
        assert_eq!(result, Ok(2286));
    }
}
