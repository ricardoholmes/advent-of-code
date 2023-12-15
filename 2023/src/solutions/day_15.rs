type Parsed = Vec<char>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let line = input_raw
        .lines()
        .next()
        .unwrap();

    let steps = line
        .split(',')
        .map(|step| step.chars().collect())
        .collect();

    Ok(steps)
}

pub fn part_one(input: &[Parsed]) -> Result<u32, String> {
    let mut total = 0;
    for step in input {
        let mut hash = 0;
        for &c in step {
            hash += c as u32;
            hash *= 17;
            hash %= 256;
        }
        total += hash;
    }
    Ok(total)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_15_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(1320));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_15_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
