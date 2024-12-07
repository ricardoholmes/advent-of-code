type Parsed = (i64, Vec<i64>); // goal, numbers (reversed)

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(
        input_raw.lines()
                 .map(|l| {
                    let (goal,ns) = l.split_once(": ").unwrap();
                    (
                        goal.parse::<i64>().unwrap(),
                        ns.split(' ').rev().map(|x| x.parse::<i64>().unwrap()).collect()
                    )
                  }).collect()
    )
}

pub fn part_one(input: &[Parsed]) -> Result<i64, String> {
    let mut total = 0;
    for (goal, ns) in input {
        if is_possible(*goal, ns, false) {
            total += goal;
        }
    }
    Ok(total)
}

pub fn part_two(input: &[Parsed]) -> Result<i64, String> {
    let mut total = 0;
    for (goal, ns) in input {
        if is_possible(*goal, ns, true) {
            total += goal;
        }
    }
    Ok(total)
}

// checks validity, walking through the list backwards
fn is_possible(goal: i64, ns: &[i64], part_2: bool) -> bool {
    if ns.is_empty() {
        return goal == 0;
    }
    else if goal < 0 {
        return false;
    }

    // order of checks is intentional to ensure fastest performance

    if part_2 {
        // deal with concatenation
        // interpret it as "a || b = a * (10^len(b)) + b"
        // where "||" is the concatenation operator
        // then, making "a" the subject of "a || b = c" gives:
        // "a = (c - b) / len(b)"

        let n = ns[0];
        let n_len = n.checked_ilog10().unwrap_or(0) + 1;
        let mult = 10_i64.pow(n_len);

        if (goal - n) % mult == 0 && is_possible((goal - n) / mult, &ns[1..], part_2) {
            return true;
        }
    }

    // more likely to fail than addition, so checking first is faster
    if goal % ns[0] == 0 && is_possible(goal / ns[0], &ns[1..], part_2) {
        // multiplication
        true
    }
    else if is_possible(goal - ns[0], &ns[1..], part_2) {
        // addition
        true
    }
    else {
        false
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
