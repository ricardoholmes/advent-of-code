const RED: usize = 0;
const GREEN: usize = 1;
const BLUE: usize = 2;

pub fn run() -> Result<(), String> {
    let input_str: Vec<&str> = include_str!("../../inputs/input_2.txt")
        .lines()
        .collect();

    // each element is: (id, [max_red_count, max_green_count, max_blue_count])
    let mut games: Vec<(u32, [u32; 3])> = vec![];

    for line in &input_str {
        let line_split: Vec<&str> = line.split(": ").collect();
        let id: u32 = line_split[0].split_ascii_whitespace().last().unwrap().parse().unwrap();
        let game = line_split[1].replace("; ", ", ");
        let game_properties = game.split(", ");

        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;
        for property in game_properties {
            let split: Vec<&str> = property.split_ascii_whitespace().collect();

            let count: u32 = match split[0].parse() {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };

            let color = split[1];

            if color == "red" && count > red_count {
                red_count = count;
            }
            if color == "green" && count > green_count {
                green_count = count;
            }
            if color == "blue" && count > blue_count {
                blue_count = count;
            }
        }

        games.push((id, [red_count, green_count, blue_count]));
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

fn part_one(games: &[(u32, [u32; 3])]) -> Result<u32, String> {
    Ok(games
        .iter()
        .fold(0, |total, game|
            total + if game.1[RED] > 12 || 
                        game.1[GREEN] > 13 ||
                        game.1[BLUE] > 14 {
                0
            } else {
                game.0
            }
        )
    )
}

fn part_two(games: &[(u32, [u32; 3])]) -> Result<u32, String> {
    Ok(games
        .iter()
        .map(|game|
            game.1[RED] * game.1[GREEN] * game.1[BLUE]
        ).sum()
    )
}
