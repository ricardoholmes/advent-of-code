use crate::safe_unpack;

pub fn run(input_raw: &str) -> Result<(), String> {
    let solutions = parse(input_raw)?;

    let answer_part_one = part_one(&solutions)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&solutions)?;
    println!("Part two: {}", answer_part_two);

    Ok(())
}

fn parse(input_raw: &str) -> Result<(u32, u32), String> {
    let lines: Vec<&str> = input_raw.lines().collect();

    // variable needed for part 1
    let mut total = 0;

    // variables needed for part 2
    let mut instances = vec![1; lines.len()];
    let mut card_count = 0;

    for i in 0..lines.len() {
        card_count += instances[i];
        let line = lines[i];
        safe_unpack!(line.split(": "), _, game);
        safe_unpack!(game.split(" | "), win, held);
        let win: Vec<u32> = win.split_ascii_whitespace().map(|i| i.parse().unwrap()).collect();
        let held: Vec<u32> = held.split_ascii_whitespace().map(|i| i.parse().unwrap()).collect();
        let mut winning = 0;
        for i in held {
            if win.contains(&i) {
                winning += 1;
            }
        }
        if winning > 0 {
            total += 1 << (winning - 1);
            for j in (i+1).min(lines.len()-1)..(i+winning+1).min(lines.len()) {
                instances[j] += instances[i];
            }
        }
    }
    Ok((total, card_count))
}

fn part_one(solutions: &(u32, u32)) -> Result<u32, String> {
    Ok(solutions.0)
}

fn part_two(solutions: &(u32, u32)) -> Result<u32, String> {
    Ok(solutions.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_4_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_one(&parsed);
        assert_eq!(result, Ok(13));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_4_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_two(&parsed);
        assert_eq!(result, Ok(30));
    }
}
