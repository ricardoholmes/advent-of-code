use crate::safe_unpack;

pub fn run(input_raw: &str) -> Result<(), String> {
    let parsed = parse(input_raw)?;

    let answer_part_one = part_one(&parsed)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&parsed)?;
    println!("Part two: {}", answer_part_two);

    Ok(())
}

fn parse(input_raw: &str) -> Result<Vec<&str>, String> {
    let lines = input_raw.lines().collect();
    Ok(lines)
}

fn part_one(lines: &[&str]) -> Result<u32, String> {
    let mut total = 0;
    for line in lines {
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
        }
    }
    Ok(total)
}

fn part_two(lines: &[&str]) -> Result<u32, String> {
    let mut instances = vec![1; lines.len()];
    let mut cards = 0;
    for i in 0..lines.len() {
        cards += instances[i];
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
            for j in (i+1).min(lines.len()-1)..(i+winning+1).min(lines.len()) {
                instances[j] += instances[i];
            }
        }
    }
    Ok(cards)
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
