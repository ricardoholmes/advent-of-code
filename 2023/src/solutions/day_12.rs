type Parsed = (Vec<usize>, Vec<usize>, Vec<usize>);

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let lines = input_raw.lines();

    let mut parsed_lines: Vec<Parsed> = vec![];
    for line in lines {
        let mut line_split = line.split_ascii_whitespace();
        let line = line_split.next().unwrap().chars();
        parsed_lines.push((
            line.clone()
                .enumerate()
                .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
                .collect(),

            line.enumerate()
                .filter_map(|(i, c)| if c == '?' { Some(i) } else { None })
                .collect(),

            line_split
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect(),
        ))
    }

    Ok(parsed_lines)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut total = 0;
    for line in input {
        let (known, unknown, groups) = line;
        let unknown_permutations = get_permutations(unknown.len());

        for permutation in unknown_permutations {
            let mut springs = known.clone();
            for i in permutation {
                springs.push(unknown[i]);
            }
            springs.sort_unstable();

            if springs.len() == 0 {
                if *groups == vec![0] {
                    total += 1;
                }
                continue;
            }

            let mut group_sizes = vec![];

            let mut previous = springs[0];
            let mut current_group_size = 1;
            for spring in &springs[1..] {
                if previous == spring - 1 {
                    current_group_size += 1;
                }
                else {
                    group_sizes.push(current_group_size);
                    current_group_size = 1;
                }
                previous = *spring;
            }
            group_sizes.push(current_group_size);

            if group_sizes == *groups {
                total += 1;
            }
        }
    }

    Ok(total)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    Ok(0)
}

fn get_permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut permutations: Vec<Vec<usize>> = vec![];
    let perms = get_permutations(n - 1);
    let perms_with_var: Vec<Vec<usize>> = perms
        .clone()
        .iter()
        .map(|perm| {
            let mut perm = perm.clone();
            perm.push(n - 1);
            perm
        })
        .collect();

    permutations.extend(perms);
    permutations.extend(perms_with_var);

    permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_12_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(21));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_12_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
