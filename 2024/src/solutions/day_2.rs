type Parsed = Vec<i32>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(
        input_raw.lines()
                 .map(|l| l.split(' ').map(|x| x.parse().unwrap()).collect())
                 .collect()
    )
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut safe_count = 0;
    for l in input {
        if check_safe(l) {
            safe_count += 1;
        }
    }
    Ok(safe_count)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    let mut safe_count = 0;
    for l in input {
        if check_safe(l) {
            safe_count += 1;
            continue;
        }

        // not safe by default
        for i in 0..l.len() {
            let mut l = l.clone();
            l.remove(i);
            if check_safe(&l) {
                safe_count += 1;
                break;
            }
        }
    }
    Ok(safe_count)
}

fn check_safe(line: &[i32]) -> bool {
    let mut pprev = line[0];
    let mut prev = line[1];
    if (prev - pprev).abs() > 3 {
        return false;
    }
    for x in line.iter().skip(2) {
        if (prev - pprev) * (x - prev) <= 0 { // one is increasing, other decreasing (or one doing neither)
            return false;
        }
        else if (x - prev).abs() > 3 {
            return false;
        }
        pprev = prev;
        prev = *x;
    }
    true
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
