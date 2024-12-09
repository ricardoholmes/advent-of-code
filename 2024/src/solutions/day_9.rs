type Parsed = Option<usize>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut out = vec![];
    for (i, c) in input_raw.trim().chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 1 {
            out.extend((0..n).map(|_| None));
        }
        else {
            out.extend((0..n).map(|_| Some(i / 2)));
        }
    }
    Ok(out)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut input = input.to_vec();

    while input.last().is_none() {
        input.pop();
    }

    let mut total = 0;
    let mut i = 0;
    while i < input.len() {
        if let Some(x) = input[i] {
            total += i * x;
            i += 1;
            continue;
        }

        input[i] = input.pop().unwrap();
        while input.last().unwrap().is_none() {
            input.pop();
        }
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
        let example = include_str!("../../examples/day_9_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(1928));
    }
}

