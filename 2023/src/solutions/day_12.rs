use std::collections::HashMap;

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
    let mut cache = HashMap::new();
    let mut total = 0;
    for line in input {
        let (pattern, groups) = line;

        let pattern: Vec<char> = pattern.chars().collect();

        total += search(&pattern, &groups, &mut cache);
    }

    Ok(total)
}

pub fn part_two(input: &[Parsed]) -> Result<u64, String> {
    let mut cache = HashMap::new();
    let mut total = 0;
    for line in input {
        let (pattern, groups) = line;

        let mut pattern: Vec<char> = format!("{pattern}?").repeat(5).chars().collect();
        pattern = pattern.get(0..pattern.len()-1).unwrap().to_owned();

        let groups: Vec<usize> = groups.repeat(5);

        total += search(&pattern, &groups, &mut cache);
    }
    Ok(total)
}

fn search(pattern: &[char], groups: &[usize], cache: &mut HashMap<(Vec<char>, Vec<usize>), u64>) -> u64 {
    if pattern.is_empty() {
        if groups.is_empty() {
            return 1;
        }
        else {
            return 0;
        }
    }

    if groups.is_empty() {
        if pattern.contains(&'#') {
            return 0;
        }
        else {
            return 1;
        }
    }

    let cache_key = (pattern.to_vec(), groups.to_vec());
    if let Some(count) = cache.get(&cache_key) {
        return *count;
    }

    let mut count = 0;
    let next_char = pattern[0];

    if next_char == '.' || next_char == '?' {
        let empty_end = pattern
            .iter()
            .skip(1)
            .position(|&c| c != '.')
            .unwrap_or(pattern.len() - 1);

        count += search(
            pattern[1..].get(empty_end..).unwrap_or_default(),
            groups,
            cache
        )
    }

    if next_char == '#' || next_char == '?' {
        let group_size = groups[0];

        let spring_group = pattern
            .get(..group_size)
            .unwrap_or_default();

        let next_char = *pattern
            .get(group_size)
            .unwrap_or(&'.');

        if next_char != '#' && !spring_group.contains(&'.') {
            count += search(
                pattern.get(group_size+1..).unwrap_or_default(),
                groups.get(1..).unwrap_or_default(),
                cache
            )
        }
    }

    cache.insert(cache_key, count);

    count
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
        assert_eq!(solution, Ok(525152));
    }
}
