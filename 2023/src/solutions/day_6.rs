use std::iter::zip;

pub fn run(input_raw: &str) -> Result<(), String> {
    let solutions = parse(input_raw)?;

    let answer_part_one = part_one(&solutions)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&solutions)?;
    println!("Part two: {}", answer_part_two);

    Ok(())
}

fn parse(input_raw: &str) -> Result<Vec<(u32, u32)>, String> {
    let mut lines = input_raw.lines();

    let mut times_raw: Vec<char> = lines
        .next()
        .unwrap()
        .chars()
        .collect();

    let mut times: Vec<u32> = vec![];
    loop {
        times_raw = times_raw.iter().skip_while(|c| c.is_numeric()).skip_while(|c| !c.is_numeric()).map(|c| c.to_owned()).collect();
        if times_raw.len() == 0 {
            break;
        }
        let time: u32 = times_raw.iter().take_while(|c| c.is_numeric()).collect::<String>().parse().unwrap();
        times.push(time);
    }

    let mut distances_raw: Vec<char> = lines
        .next()
        .unwrap()
        .chars()
        .collect();

    let mut distances: Vec<u32> = vec![];
    loop {
        distances_raw = distances_raw.iter().skip_while(|c| c.is_numeric()).skip_while(|c| !c.is_numeric()).map(|c| c.to_owned()).collect();
        if distances_raw.len() == 0 {
            break;
        }
        let distance: u32 = distances_raw.iter().take_while(|c| c.is_numeric()).collect::<String>().parse().unwrap();
        distances.push(distance);
    }

    Ok(zip(times, distances).collect())
}

fn part_one(solutions: &[(u32, u32)]) -> Result<u32, String> {
    let mut total = 1;
    for (time, distance) in solutions {
        let mut min_time = None;
        for i in 1..*time {
            if i * (time - i) > *distance {
                min_time = Some(i);
                break;
            }
        }
        if let Some(min_time) = min_time {
            let num_ways = (*time - min_time + 1) - min_time;
            total *= num_ways;
        }
    }
    Ok(total)
}

fn part_two(solutions: &[(u32, u32)]) -> Result<u32, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_6_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_one(&parsed);
        assert_eq!(result, Ok(288));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_6_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_two(&parsed);
        assert_eq!(result, Ok(0));
    }
}
