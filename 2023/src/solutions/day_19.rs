use std::collections::HashMap;

use crate::{safe_unpack, try_get_ok};

type Workflow = Vec<(Condition, String)>;
type Part = [u64; 4];
type Parsed = (HashMap<String, Workflow>, Vec<Part>);
type Range = [(u64, u64); 4];

#[derive(Clone, Debug)]
pub enum Condition {
    None,
    LessThan(usize, u64),
    GreaterThan(usize, u64),
}

impl Condition {
    fn eval(&self, parts: &[u64]) -> bool {
        match *self {
            Condition::GreaterThan(cat, n) => parts[cat] > n,
            Condition::LessThan(cat, n) => parts[cat] < n,
            Condition::None => true,
        }
    }
}

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    safe_unpack!(input_raw.split("\n\n"), workflow_lines, part_lines);

    let mut workflows = HashMap::new();
    for line in workflow_lines.lines() {
        let desc_start = line.chars().position(|c| c == '{').unwrap();
        let name = line[..desc_start].to_string();
        let desc = &line[desc_start+1..line.len()-1];
        let mut conds = vec![];
        for cat in desc.split(',') {
            if cat.contains('>') {
                safe_unpack!(cat.split(&['>', ':']), name, value, dest);
                let value = try_get_ok!(value.parse::<u64>());
                conds.push((Condition::GreaterThan(match name {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => return Err(format!("Invalid input.")),
                }, value), dest.to_string()));
            }
            else if cat.contains('<') {
                safe_unpack!(cat.split(&['<', ':']), name, value, dest);
                let value = try_get_ok!(value.parse::<u64>());
                conds.push((Condition::LessThan(match name {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => return Err(format!("Invalid input.")),
                }, value), dest.to_string()));
            }
            else {
                conds.push((Condition::None, cat.to_string()))
            }
        }
        workflows.insert(name, conds);
    }

    let mut parts = vec![];
    for line in part_lines.lines() {
        let line = &line[1..line.len()-1];
        let mut categories = HashMap::new();
        for category in line.split(',') {
            safe_unpack!(category.split('='), name, value);
            let value = try_get_ok!(value.parse::<u64>());
            categories.insert(name, value);
        }
        parts.push([
            *categories.get("x").unwrap(),
            *categories.get("m").unwrap(),
            *categories.get("a").unwrap(),
            *categories.get("s").unwrap(),
        ]);
    }

    Ok((workflows, parts))
}

pub fn part_one(input: &Parsed) -> Result<u64, String> {
    let (workflows, parts) = input;

    let mut total = 0;
    for part in parts {
        let mut workflow_name = String::from("in");
        while workflow_name != "A" && workflow_name != "R" {
            let workflow = workflows.get(&workflow_name).unwrap();
            let mut found = false;
            for (cond, dest) in workflow {
                if cond.eval(part) {
                    found = true;
                    workflow_name = dest.clone();
                    break;
                }
            }
            if !found {
                panic!();
            }
        }
        if workflow_name == "A" {
            total += part.iter().sum::<u64>();
        }
    }

    Ok(total)
}

pub fn part_two(input: &Parsed) -> Result<u64, String> {
    let (workflows, _) = input;

    let mut ranges = vec![];
    ranges.push(([
        (1, 4000),
        (1, 4000),
        (1, 4000),
        (1, 4000),
    ], "in".to_string()));

    let mut total = 0;
    while !ranges.is_empty() {
        let (mut range, workflow_name) = ranges.pop().unwrap();

        if workflow_name == "A" {
            let num_combinations = range
                .iter()
                .fold(1, |count, r| count * (r.1 + 1 - r.0));

            total += num_combinations;
            continue;
        }
        else if workflow_name == "R" {
            continue;
        }

        let workflow = workflows.get(&workflow_name).unwrap();
        for (cond, dest) in workflow {
            let (range_passed, range_failed) = split_by_cond(&range, cond);

            match range_passed {
                Some(next_range) => ranges.push((next_range, dest.clone())),
                None => (),
            };

            match range_failed {
                Some(next_range) => range = next_range,
                None => break,
            };
        }
    }

    Ok(total)
}

fn split_by_cond(range: &Range, cond: &Condition) -> (Option<Range>, Option<Range>) {
    match *cond {
        Condition::GreaterThan(cat, n) => {
            let (lower, upper) = range[cat];

            if upper <= n {
                (None, Some(*range))
            } else if lower > n {
                (Some(*range), None)
            } else {
                let mut first_range = range.clone();
                let mut second_range = range.clone();

                first_range[cat].0 = n + 1;
                second_range[cat].1 = n;

                (Some(first_range), Some(second_range))
            }
        },
        Condition::LessThan(cat, n) => {
            let (lower, upper) = range[cat];

            if lower >= n {
                (None, Some(*range))
            } else if upper < n {
                (Some(*range), None)
            } else {
                let mut first_range = range.clone();
                let mut second_range = range.clone();

                first_range[cat].1 = n - 1;
                second_range[cat].0 = n;

                (Some(first_range), Some(second_range))
            }
        },
        Condition::None => (Some(*range), None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_19_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(19114));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_19_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(167409079868000));
    }
}
