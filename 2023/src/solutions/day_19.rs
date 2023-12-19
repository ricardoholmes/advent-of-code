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

#[derive(Copy, Clone)]
struct CategoryRanges {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl CategoryRanges {
    fn count(&self) -> u64 {
        let x_count = self.x.1 + 1 - self.x.0;
        let m_count = self.m.1 + 1 - self.m.0;
        let a_count = self.a.1 + 1 - self.a.0;
        let s_count = self.s.1 + 1 - self.s.0;

        x_count * m_count * a_count * s_count
    }

    fn split_by_cond(&self, cond: &Condition) -> (Option<CategoryRanges>, Option<CategoryRanges>) {
        match cond {
            Condition::GreaterThan(cat, n) => {
                let (lower, upper) = match cat.as_str() {
                    "x" => self.x,
                    "m" => self.m,
                    "a" => self.a,
                    "s" => self.s,
                    _ => panic!(),
                };

                if upper <= *n {
                    (None, Some(*self))
                } else if lower > *n {
                    (Some(*self), None)
                } else {
                    let mut first_range = self.clone();
                    let mut second_range = self.clone();
                    match cat.as_str() {
                        "x" => {
                            first_range.x.0 = n + 1;
                            second_range.x.1 = *n;
                        },
                        "m" => {
                            first_range.m.0 = n + 1;
                            second_range.m.1 = *n;
                        },
                        "a" => {
                            first_range.a.0 = n + 1;
                            second_range.a.1 = *n;
                        },
                        "s" => {
                            first_range.s.0 = n + 1;
                            second_range.s.1 = *n;
                        },
                        _ => panic!(),
                    };

                    (Some(first_range), Some(second_range))
                }
            },
            Condition::LessThan(cat, n) => {
                let (lower, upper) = match cat.as_str() {
                    "x" => self.x,
                    "m" => self.m,
                    "a" => self.a,
                    "s" => self.s,
                    _ => panic!(),
                };

                if lower >= *n {
                    (None, Some(*self))
                } else if upper < *n {
                    (Some(*self), None)
                } else {
                    let mut first_range = self.clone();
                    let mut second_range = self.clone();
                    match cat.as_str() {
                        "x" => {
                            first_range.x.1 = n - 1;
                            second_range.x.0 = *n;
                        },
                        "m" => {
                            first_range.m.1 = n - 1;
                            second_range.m.0 = *n;
                        },
                        "a" => {
                            first_range.a.1 = n - 1;
                            second_range.a.0 = *n;
                        },
                        "s" => {
                            first_range.s.1 = n - 1;
                            second_range.s.0 = *n;
                        },
                        _ => panic!(),
                    };

                    (Some(first_range), Some(second_range))
                }
            },
            Condition::None => (Some(*self), None),
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
    ranges.push((CategoryRanges {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    }, "in".to_string()));

    let mut total = 0;
    while !ranges.is_empty() {
        let (mut range, workflow_name) = ranges.pop().unwrap();
        if workflow_name == "A" {
            total += range.count();
            continue;
        }
        else if workflow_name == "R" {
            continue;
        }
        let workflow = workflows.get(&workflow_name).unwrap();
        for (cond, dest) in workflow {
            let (range_passed, range_failed) = range.split_by_cond(cond);

            match range_passed {
                Some(next_range) => ranges.push((next_range, dest.clone())),
                None => ()
            };

            match range_failed {
                Some(next_range) => range = next_range,
                None => break,
            };
        }
    }

    Ok(total)
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
