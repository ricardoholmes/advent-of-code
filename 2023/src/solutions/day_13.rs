use std::collections::{HashSet, HashMap};

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
                let left = line[..=line_pos].chars().rev().collect::<String>();
                let left = left.as_str();

                let right = &line[line_pos+1..];

                let refl_len = left.len().min(right.len());
                if left[..refl_len] != right[..refl_len] {
                    vertical_lines.remove(&line_pos);
                }
            }
        }
        if vertical_lines.len() == 1 {
            total += vertical_lines.iter().next().unwrap() + 1;
            continue;
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
    let mut total = 0;
    for pattern in input {
        if let Some(n) = get_vertical_reflection(pattern) {
            total += n;
        }
        
        let transposed = transpose(pattern
            .iter()
            .map(|n| n.chars().collect())
            .collect()
        );

        let pattern: Vec<String> = transposed
            .iter()
            .map(|line| line.iter().collect())
            .collect();

        if let Some(n) = get_vertical_reflection(&pattern) {
            total += n * 100;
            continue;
        }
    }

    Ok(total)
}

fn get_vertical_reflection(pattern: &[String]) -> Option<usize> {
    let mut vertical_lines: HashMap<usize, u8> = (0..(pattern[0].len() - 1))
        .map(|n| (n, 0))
        .collect();

    for line in pattern {
        if vertical_lines.len() == 0 {
            break;
        }

        for &line_pos in vertical_lines.clone().keys() {
            let left = line[..=line_pos].chars().rev().collect::<String>();
            let right = &line[line_pos+1..];

            let refl_len = left.len().min(right.len());
            let left = &left[..refl_len];
            let right = &right[..refl_len];
            if left != right {
                let smudges = *vertical_lines.get(&line_pos).unwrap();

                if smudges > 0 {
                    vertical_lines.remove(&line_pos);
                    continue;
                }

                let mut differences = 0;
                for i in 0..refl_len {
                    if left[i..i+1] != right[i..i+1] {
                        differences += 1;
                    }
                }

                if differences != 1 {
                    vertical_lines.remove(&line_pos);
                    continue;
                }

                vertical_lines.insert(line_pos, smudges + 1);
            }
        }
    }
    vertical_lines = vertical_lines.iter().filter(|(_, &v)| v == 1).map(|(&k, &v)| (k, v)).collect();

    if vertical_lines.len() != 1 {
        None
    }
    else {
        Some(*vertical_lines.keys().next().unwrap() as usize + 1)
    }
}

fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    let mut transposed: Vec<Vec<T>> = vec![];
    for line in matrix {
        for (i, c) in line.iter().enumerate() {
            match transposed.get(i) {
                Some(element) => {
                    let mut transposed_line = element.clone();
                    transposed_line.push(c.clone());
                    transposed[i] = transposed_line;
                }
                None => transposed.push(vec![c.clone()]),
            }
        }
    }
    transposed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let nums = vec![vec![1,2], vec![3,4]];
        let goal = vec![vec![1,3], vec![2,4]];
        assert_eq!(transpose(nums), goal);
    }

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_13_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(709));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_13_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(1400));
    }
}
