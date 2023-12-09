pub fn parse(input_raw: &str) -> Result<Vec<Vec<i32>>, String> {
    let lines = input_raw
        .lines()
        .map(|line| line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
        )
        .collect();

    Ok(lines)
}

pub fn part_one(input: &[Vec<i32>]) -> Result<i32, String> {
    let mut total = 0;
    for line in input {
        let mut differences = vec![line.clone()];
        while !differences.iter().last().unwrap().iter().all(|&n| n == 0) {
            differences.push(get_differences(&differences.iter().last().unwrap()));
        }

        let mut end = 0;
        for i in differences.iter().rev() {
            end += i.iter().last().unwrap();
        }
        total += end;
    }

    Ok(total)
}

pub fn part_two(input: &[Vec<i32>]) -> Result<i32, String> {
    let mut total = 0;
    for line in input {
        let mut differences = vec![line.clone()];
        while !differences.iter().last().unwrap().iter().all(|&n| n == 0) {
            differences.push(get_differences(&differences.iter().last().unwrap()));
        }

        let mut start = 0;
        for i in differences.iter().rev() {
            let diff_start = i.iter().next().unwrap();
            start = diff_start - start;
        }
        total += start;
    }

    Ok(total)
}

fn get_differences(line: &[i32]) -> Vec<i32> {
    let mut prev = line[0];
    let mut differences = vec![];
    for i in line.iter().skip(1) {
        differences.push(i - prev);
        prev = *i;
    }
    differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_9_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(114));
    }

    #[test]
    fn test_part2() {
        let example: &str = include_str!("../../examples/day_9_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(2));
    }
}
