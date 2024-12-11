use std::collections::HashMap;

type Parsed = usize;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(
        input_raw.trim().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect()
    )
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut stones = input.to_vec();

    // iterate for each blink
    for _ in 0..25 {
        let mut new_stones = vec![];
        for stone in stones {
            // if engraved with 0, replaced by stone with 1
            if stone == 0 {
                new_stones.push(1);
                continue;
            }

            // if stone's length is even, split it in two
            let n_len = stone.checked_ilog10().unwrap_or(0) + 1;
            if n_len % 2 == 0 {
                let factor_10 = 10_usize.pow(n_len / 2);
                new_stones.push(stone / factor_10);
                new_stones.push(stone % factor_10);
                continue;
            }

            // otherwise, multiply stone by 2024
            new_stones.push(stone * 2024);
        }
        stones = new_stones;
    }

    Ok(stones.len())
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    let mut stones = HashMap::new();
    for &i in input {
        *stones.entry(i).or_default() += 1
    }

    // iterate for each blink
    for _ in 0..75 {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones {
            // if engraved with 0, replaced by stone with 1
            if stone == 0 {
                *new_stones.entry(1).or_default() += count;
                continue;
            }

            // if stone's length is even, split it in two
            let n_len = stone.checked_ilog10().unwrap_or(0) + 1;
            if n_len % 2 == 0 {
                let factor_10 = 10_usize.pow(n_len / 2);
                *new_stones.entry(stone / factor_10).or_default() += count;
                *new_stones.entry(stone % factor_10).or_default() += count;
                continue;
            }

            // otherwise, multiply stone by 2024
            *new_stones.entry(stone * 2024).or_default() += count;
        }
        stones = new_stones;
    }

    Ok(stones.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_11_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(55312));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_11_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
