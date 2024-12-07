type Parsed = (i64, Vec<i64>);

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(
        input_raw.lines()
                 .map(|l| {
                    let (goal,ns) = l.split_once(": ").unwrap();
                    (
                        goal.parse::<i64>().unwrap(),
                        ns.split(' ').map(|x| x.parse::<i64>().unwrap()).collect()
                    )
                  }).collect()
    )
}

pub fn part_one(input: &[Parsed]) -> Result<i64, String> {
    let mut total = 0;
    for (goal, ns) in input {
        // println!("\n{goal}");
        let ns_rev: Vec<i64> = ns.iter().rev().map(|&x| x).collect();
        if is_possible(*goal, &ns_rev) {
            // println!("----- {goal} {ns:?} -----");
            total += goal;
        }
    }
    Ok(total)
}

fn is_possible(goal: i64, ns: &[i64]) -> bool {
    // println!("{goal} {ns:?}");
    if ns.is_empty() {
        goal == 0
    }
    else {
        is_possible(goal - ns[0], &ns[1..]) || (goal % ns[0] == 0 && is_possible(goal / ns[0], &ns[1..]))
    }
}

pub fn part_two(input: &[Parsed]) -> Result<i64, String> {
    Ok(0)
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
        assert_eq!(solution, Ok(0));
    }
}
