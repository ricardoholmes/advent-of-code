type Parsed = (i64, Vec<String>);

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(
        input_raw.lines()
                 .map(|l| {
                    let (goal,ns) = l.split_once(": ").unwrap();
                    (
                        goal.parse::<i64>().unwrap(),
                        ns.split(' ').map(|x| x.to_string()).collect()
                    )
                  }).collect()
    )
}

pub fn part_one(input: &[Parsed]) -> Result<i64, String> {
    let mut total = 0;
    for (goal, ns) in input {
        let ns_rev: Vec<i64> = ns.iter().rev().map(|x| x.parse::<i64>().unwrap()).collect();
        if is_possible_p1(*goal, &ns_rev) {
            total += goal;
        }
    }
    Ok(total)
}

fn is_possible_p1(goal: i64, ns_rev: &[i64]) -> bool {
    if ns_rev.is_empty() {
        goal == 0
    }
    else if is_possible_p1(goal - ns_rev[0], &ns_rev[1..]) {
        true
    }
    else if goal % ns_rev[0] == 0 {
        is_possible_p1(goal / ns_rev[0], &ns_rev[1..])
    }
    else {
        false
    }
}

pub fn part_two(input: &[Parsed]) -> Result<i64, String> {
    let mut total = 0;
    for (goal, ns) in input {
        if is_possible_p2(ns[0].parse().unwrap(), &ns[1..], goal) {
            total += goal;
        }
    }
    Ok(total)
}

fn is_possible_p2(n: i64, ns: &[String], goal: &i64) -> bool {
    if ns.is_empty() {
        n == *goal
    }
    else {
        let x = ns[0].parse::<i64>().unwrap();
        if is_possible_p2(n + x, &ns[1..], goal) {
            true
        }
        else if is_possible_p2(n * x, &ns[1..], goal) {
            true
        }
        else {
            let lshift = (10 as i64).pow(x.to_string().len() as u32);
            let n = (n * lshift) + x;
            is_possible_p2(n, &ns[1..], goal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_7_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(3749));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_7_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(11387));
    }
}
