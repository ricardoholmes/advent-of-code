use crate::safe_unpack;

#[derive(PartialEq, Debug)]
pub struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

pub fn parse(input_raw: &str) -> Result<Vec<Game>, String> {
    let lines = input_raw.lines();

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
                Err(e) => return Err(format!("{e:?} - '{count}'")),
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

pub fn part_one(games: &[Game]) -> Result<u32, String> {
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

pub fn part_two(games: &[Game]) -> Result<u32, String> {
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
        let example = include_str!("../../examples/day_2_1.txt");

        let parsed = parse(example).unwrap();
        let result = part_one(&parsed);
        assert_eq!(result, Ok(8));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_2_1.txt");

        let parsed = parse(example).unwrap();
        let result = part_two(&parsed);
        assert_eq!(result, Ok(2286));
    }
}
