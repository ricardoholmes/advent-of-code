#[derive(Clone, Copy, Debug)]
pub enum Token {
    Mul(u32),
    Do,
    Dont
}

type Parsed = Token;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut tokenized = vec![];
    let mut start = 0;
    while start < input_raw.len() {
        if let Some(idx) = input_raw[start..].find(&['d', 'm']) {
            start += idx;
        } else {
            break;
        }

        if input_raw[start..].starts_with("do()") {
            tokenized.push(Token::Do);
            start += 4;
        }
        else if input_raw[start..].starts_with("don't()") {
            tokenized.push(Token::Dont);
            start += 7;
        }
        else if input_raw[start..].starts_with("mul(") {
            start += 4;
            let Some(idx) = input_raw[start..].find(|c| c == ')') else {
                continue;
            };

            let mut split = input_raw[start..start+idx].split(',');
            let Some(x) = split.next() else { continue; };
            let Some(y) = split.next() else { continue; };
            if split.next().is_some() {
                continue;
            }

            let Ok(x) = x.parse::<u32>() else { continue; };
            let Ok(y) = y.parse::<u32>() else { continue; };
            tokenized.push(Token::Mul(x * y));
            start += idx;
        }
        else {
            start += 1;
        }
    }

    Ok(tokenized)
}

pub fn part_one(input: &[Parsed]) -> Result<u32, String> {
    let mut total = 0;
    for tok in input {
        if let Token::Mul(x) = tok {
            total += x;
        }
    }
    Ok(total)
}

pub fn part_two(input: &[Parsed]) -> Result<u32, String> {
    let mut total = 0;
    let mut enabled = true;
    for &tok in input {
        match tok {
            Token::Do => enabled = true,
            Token::Dont => enabled = false,
            Token::Mul(x) => if enabled { total += x },
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_3_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(161));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_3_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(48));
    }
}
