use std::collections::{HashMap, HashSet};

type Parsed = (HashMap<usize,HashSet<usize>>, Vec<Vec<usize>>);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut input = input_raw.lines();
    let rules = input.by_ref().take_while(|x| !x.is_empty()).map(|x| {
        let (a, b) = x.split_once('|').unwrap();
        (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
    });

    let mut rules_dict = HashMap::new();
    for (k, v) in rules {
        rules_dict.entry(k).or_insert(HashSet::new()).insert(v);
    }

    let _ = input.by_ref().skip_while(|x| x.is_empty());
    let pages = input.map(|x| x.split(',').map(|n| n.parse::<usize>().unwrap()).collect()).collect();

    Ok((rules_dict, pages))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (rules, pages) = input.clone();

    let mut total = 0;
    for page in pages {
        if is_valid(&rules, &page) {
            total += page[page.len() / 2]
        }
    }

    Ok(total)
}

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    let (rules, pages) = input.clone();

    let mut total = 0;
    for page in pages {
        if is_valid(&rules, &page) {
            continue;
        }

        let mut page = page.clone();
        let mut valid = false;
        while !valid {
            valid = true;
            let mut nums_past = HashSet::new();
            for i in 0..page.len() {
                let num = page[i];
                if let Some(nums_must_be_after) = rules.get(&num) {
                    if let Some(&x) = nums_past.intersection(nums_must_be_after).next() {
                        let j = page.iter().position(|&n| n == x).unwrap();
                        page.swap(i, j);
                        valid = false;
                    }
                }
                nums_past.insert(num);
            }
        }

        total += page[page.len() / 2];
    }

    Ok(total)
}

fn is_valid(rules: &HashMap<usize,HashSet<usize>>, page: &[usize]) -> bool {
    let mut valid = true;
    let mut nums_past = HashSet::new();
    for &num in page {
        if let Some(nums_must_be_after) = rules.get(&num) {
            if let Some(_) = nums_past.intersection(nums_must_be_after).next() {
                valid = false;
                break;
            }
        }
        nums_past.insert(num);
    }
    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_5_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(143));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_5_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(123));
    }
}
