use std::{cmp::Ordering, collections::HashSet};

type Parsed = (HashSet<(usize,usize)>, Vec<Vec<usize>>);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut input = input_raw.lines();
    let rules: HashSet<(usize, usize)> = input.by_ref().take_while(|x| !x.is_empty()).map(|x| {
        let (a, b) = x.split_once('|').unwrap();
        (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
    }).collect();

    let _ = input.by_ref().skip_while(|x| x.is_empty());

    let pages = input
            .map(|x| x.split(',').map(|n| n.parse::<usize>().unwrap()).collect())
            .collect();

    Ok((rules, pages))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (rules, pages) = input.clone();

    let mut total = 0;
    for page in pages {
        if page.is_sorted_by(|&a, &b| rules.contains(&(a,b))) {
            total += page[page.len() / 2];
        }
    }

    Ok(total)
}

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    let (rules, pages) = input.clone();

    let mut total = 0;
    for mut page in pages {
        if page.is_sorted_by(|&a, &b| rules.contains(&(a,b))) {
            continue;
        }

        page.sort_unstable_by(|&a, &b|
            if rules.contains(&(a,b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        );

        total += page[page.len() / 2];
    }

    Ok(total)
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
