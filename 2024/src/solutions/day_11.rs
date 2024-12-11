type Parsed = usize;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(
        input_raw.trim().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect()
    )
}


/*
If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
*/


pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut stones = input.to_vec();
    println!("Initial arrangement:");
    println!("{}\n", stones.iter().map(|x| x.to_string() + " ").collect::<String>());

    // iterate for each blink
    for i in 0..25 {
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

        println!("{}: {}", i+1, stones.len());
        // println!("After {} blinks:", i+1);
        // println!("{}\n", stones.iter().map(|x| x.to_string() + " ").collect::<String>());
    }

    Ok(stones.len())
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
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
