use std::collections::HashMap;

use crate::{safe_unpack, try_get_ok};

type Workflow = Vec<(Condition, String)>;
type Part = [u64;4];
type Parsed = (HashMap<String, Workflow>, Vec<Part>);

#[derive(Clone, Debug)]
pub enum Condition {
    None,
    LessThan(String, u64),
    GreaterThan(String, u64),
}

impl Condition {
    fn eval(&self, parts: &[u64]) -> bool {
        match self {
            Condition::GreaterThan(cat, n) => (match cat.as_str() {
                "x" => parts[0],
                "m" => parts[1],
                "a" => parts[2],
                "s" => parts[3],
                _ => panic!(),
            }) > *n,
            Condition::LessThan(cat, n) => (match cat.as_str() {
                "x" => parts[0],
                "m" => parts[1],
                "a" => parts[2],
                "s" => parts[3],
                _ => panic!(),
            }) < *n,
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
                conds.push((Condition::GreaterThan(name.to_string(), value), dest.to_string()));
            }
            else if cat.contains('<') {
                safe_unpack!(cat.split(&['<', ':']), name, value, dest);
                let value = try_get_ok!(value.parse::<u64>());
                conds.push((Condition::LessThan(name.to_string(), value), dest.to_string()));
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

    // println!("{workflows:?}");
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

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    Ok(0)
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
        assert_eq!(solution, Ok(0));
    }
}
