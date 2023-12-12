type Parsed = (String, Vec<usize>);

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let lines = input_raw.lines();

    let mut parsed_lines: Vec<Parsed> = vec![];
    for line in lines {
        let mut line_split = line.split_ascii_whitespace();
        let pattern = line_split.next().unwrap().to_string();

        parsed_lines.push((
            pattern,

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

pub fn part_one(input: &[Parsed]) -> Result<u64, String> {
    let mut total = 0;
    for (line_index, line) in input.iter().enumerate() {
        let (pattern, groups) = line;

        let pattern: Vec<char> = pattern.chars().collect();

        let start = total;
        total += search(&pattern, &groups);

        println!("{}/{} - {}", line_index + 1, input.len(), total - start);
    }

    Ok(total)
}

pub fn part_two(input: &[Parsed]) -> Result<u64, String> {
    let mut total = 0;
    for (line_index, line) in input.iter().enumerate() {
        let (pattern, groups) = line;

        let pattern: Vec<char> = pattern.repeat(5).chars().collect();
        let groups: Vec<usize> = groups.repeat(5);

        let start = total;
        total += search(&pattern, &groups);

        println!("{}/{} - {}", line_index + 1, input.len(), total - start);
    }
    Ok(total)
}

fn search(pattern: &[char], groups: &[usize]) -> u64 {
    if pattern.is_empty() {
        if groups.is_empty() {
            return 1;
        }
        else {
            return 0;
        }
    }

    let next_char = pattern[0];
    if next_char == '.' {
        let empty_end = pattern
            .iter()
            .position(|&c| c != '.')
            .unwrap_or(pattern.len());

        search(
            pattern.get(empty_end..).unwrap_or_default(),
            groups
        )
    }
    else if next_char == '#' {
        let spring_end = pattern
            .iter()
            .position(|&c| c != '#')
            .unwrap_or(pattern.len());

        match groups.get(0) {
            Some(&group) => if spring_end > group {
                0
            } else {
                let spring_end = match pattern.iter().skip(spring_end).position(|&c| c != '?') {
                    Some(index) => index + spring_end,
                    None => pattern.len(),
                };

                if spring_end >= group {
                    search(
                        pattern.get(group+1..).unwrap_or_default(),
                        groups.get(1..).unwrap_or_default()
                    )
                } else if pattern.get(spring_end).is_some_and(|&c| c == '#') {
                    search(
                        &[
                            &['#'].repeat(spring_end),
                            pattern.get(spring_end..).unwrap_or_default()
                        ].concat(),
                        groups
                    )
                } else {
                    0
                }
            },
            None => 0,
        }
    }
    else if next_char == '?' {
        let rest_of_pattern = pattern.get(1..).unwrap_or_default();
        let is_spring = [&['#'], rest_of_pattern].concat();

        search(&is_spring, groups) + search(rest_of_pattern, groups)
    }
    else {
        panic!("Invalid character in pattern!");
    }
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

    // #[test]
    // fn test_part2() {
    //     let example = include_str!("../../examples/day_12_1.txt");
    //     let parsed = parse(example).unwrap();
    //     let solution = part_two(&parsed);
    //     assert_eq!(solution, Ok(525152));
    // }
}
