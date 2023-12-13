use std::collections::HashSet;

type Parsed = Vec<String>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut patterns: Vec<Parsed> = vec![];
    let mut pattern: Parsed = vec![];
    for line in input_raw.lines() {
        if line.is_empty() {
            patterns.push(pattern.clone());
            pattern.clear();
            continue;
        }

        pattern.push(line.to_string());
    }

    if !pattern.is_empty() {
        patterns.push(pattern);
    }

    Ok(patterns)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut total = 0;
    for pattern in input {
        let mut vertical_lines: HashSet<usize> = (0..(pattern[0].len() - 1)).collect();
        for line in pattern {
            if vertical_lines.len() == 0 {
                break;
            }

            for line_pos in vertical_lines.clone() {
                let left = &line[..=line_pos].chars().rev().collect::<String>();
                let left = left.as_str();

                let right = &line[line_pos+1..];

                let refl_len = left.len().min(right.len());
                if left[..refl_len] != right[..refl_len] {
                    vertical_lines.remove(&line_pos);
                }
            }
        }
        if vertical_lines.len() > 1 {
            panic!("MULTIPLE REFLECTIONS?????");
        }
        else if vertical_lines.len() == 1 {
            total += vertical_lines.iter().next().unwrap() + 1;
        }

        let mut previous_lines = vec![];
        for i in 0..pattern.len()-1 {
            previous_lines.insert(0, pattern[i].clone());
            let reflected_pattern = &pattern[i+1..];

            let refl_len = previous_lines.len().min(reflected_pattern.len());
            if previous_lines[..refl_len] == reflected_pattern[..refl_len] {
                total += 100 * (i + 1);
                break;
            }
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
        let example = include_str!("../../examples/day_13_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(405));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_13_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
