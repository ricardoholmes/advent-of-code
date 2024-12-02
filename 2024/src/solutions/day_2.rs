type Parsed = Vec<i32>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(
        input_raw.lines()
                 .map(|l| l.split(' ').map(|x| x.parse().unwrap()).collect())
                 .collect()
    )
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut unsafe_count = 0;
    for l in input {
        let mut pprev = l[0];
        let mut prev = l[1];
        if (prev - pprev).abs() > 3 {
            unsafe_count += 1;
            continue;
        }
        for x in l.iter().skip(2) {
            if (prev - pprev) * (x - prev) <= 0 { // one is increasing, other decreasing (or one doing neither)
                unsafe_count += 1;
                break;
            }
            else if (x - prev).abs() > 3 {
                unsafe_count += 1;
                break;
            }
            pprev = prev;
            prev = *x;
        }
    }
    Ok(input.len() - unsafe_count)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_2_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(2));
    }
}
