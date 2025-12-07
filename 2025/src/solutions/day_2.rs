type Parsed = (u64, u64);

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut parsed = vec![];
    for range in input_raw.trim().split(',') {
        let mut split = range.split('-');

        let lower_s = split.next().unwrap();
        let higher_s = split.next().unwrap();

        if lower_s.len() == higher_s.len() {
            parsed.push((
                lower_s.parse().unwrap(), 
                higher_s.parse().unwrap()
            ));
            continue;
        }

        let mut lower = lower_s.parse().unwrap();
        for len in lower_s.len()..higher_s.len() {
            let higher = 10_u64.pow(len as u32) - 1;
            parsed.push((
                lower, 
                higher
            ));
            lower = higher + 1;
        }
        parsed.push((
            lower, 
            higher_s.parse().unwrap()
        ));
    }

    Ok(parsed)
}

pub fn part_one(input: &[Parsed]) -> Result<u64, String> {
    let mut sum = 0;
    for &(start, end) in input {
        let start_str = start.to_string();
        if start_str.len() % 2 != 0 {
            continue;
        }
        let half_len = (start_str.len() / 2) as u32;
        let increment = 10_u64.pow(half_len) + 1;
        let mut n = start.div_ceil(increment) * increment;
        while n <= end {
            sum += n;
            n += increment;
        }
    }
    Ok(sum)
}

pub fn part_two(input: &[Parsed]) -> Result<u64, String> {
    let mut sum = 0;
    for &(start, end) in input {
        let strlen = start.to_string().len();
        let mut found = vec![];
        for factor in (2..=strlen).filter(|&f| strlen % f == 0) {
            let m = 10_u64.pow((strlen / factor) as u32);
            let increment = (1..factor).fold(1, |acc, _| acc * m + 1);
            let mut n = start.div_ceil(increment) * increment;
            while n <= end {
                if !found.contains(&n) {
                    found.push(n);
                    sum += n;
                }
                n += increment;
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_2_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(1227775554));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_2_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(4174379265));
    }
}
