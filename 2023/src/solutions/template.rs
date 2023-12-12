type Parsed = ();

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Err(format!("Not implemented"))
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_xx_x.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(0));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_xx_x.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
