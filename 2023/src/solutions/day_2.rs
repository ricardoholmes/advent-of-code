use crate::safe_unpack;

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

pub fn run(input_raw: &str) -> Result<(), String> {
    let input: Vec<&str> = input_raw.lines().collect();

    let games = match parse_input(&input) {
        Ok(games) => games,
        Err(e) => return Err(e),
    };

    let answer_part_one = part_one(&games)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&games)?;
    println!("Part two: {}", answer_part_two);

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
        let example: Vec<&str> = include_str!("../../examples/day_2_1.txt")
            .lines()
            .collect();

        let parsed = parse_input(&example).unwrap();
        let result = part_one(&parsed);
        assert_eq!(result, Ok(8));
    }

    #[test]
    fn test_part2() {
        let example: Vec<&str> = include_str!("../../examples/day_2_1.txt")
            .lines()
            .collect();

        let parsed = parse_input(&example).unwrap();
        let result = part_two(&parsed);
        assert_eq!(result, Ok(2286));
    }
}
