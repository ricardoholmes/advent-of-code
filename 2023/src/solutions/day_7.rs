use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn run(input_raw: &str) -> Result<(), String> {
    let solutions = parse(input_raw)?;

    let answer_part_one = part_one(&solutions)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&solutions)?;
    println!("Part two: {}", answer_part_two);

    Ok(())
}

fn parse(input_raw: &str) -> Result<Vec<(&str, u32)>, String> {
    let lines_parsed: Vec<(&str, u32)> = input_raw
        .lines()
        .map(|line| {
            let mut line_split = line.split_ascii_whitespace();
            
            (
                line_split.next().unwrap(),
                line_split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    Ok(lines_parsed)
}

fn part_one(input: &[(&str, u32)]) -> Result<u32, String> {
    let mut scored: Vec<(HandType, Vec<u32>, u32)> = vec![];

    for (hand, bid) in input {
        let hand_type = get_hand_type(hand);
        let hand_values: Vec<u32> = hand.chars().map(|c| if c.is_ascii_digit() {
                c.to_digit(10).unwrap()
            } else {
                match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!(),
                }
            })
            .collect();

        scored.push((hand_type, hand_values, *bid));
    }

    scored.sort_unstable_by(|a, b| match a.0.partial_cmp(&b.0).unwrap() {
        Ordering::Equal => a.1.partial_cmp(&b.1).unwrap(),
        ord => ord,
    });

    let total_winnings = scored
        .iter()
        .enumerate()
        .fold(0, |total, (index, hand)| total + hand.2 * (1 + index as u32));

    Ok(total_winnings)
}

fn part_two(input: &[(&str, u32)]) -> Result<u32, String> {
    let mut scored: Vec<(HandType, Vec<u32>, u32)> = vec![];

    for (hand, bid) in input {
        let hand_type = get_hand_type_with_joker(hand);
        let hand_values: Vec<u32> = hand.chars().map(|c| if c.is_ascii_digit() {
                c.to_digit(10).unwrap()
            } else {
                match c {
                    'J' => 1,
                    'T' => 10,
                    'Q' => 11,
                    'K' => 12,
                    'A' => 13,
                    _ => panic!(),
                }
            })
            .collect();

        scored.push((hand_type, hand_values, *bid));
    }

    scored.sort_unstable_by(|a, b| match a.0.partial_cmp(&b.0).unwrap() {
        Ordering::Equal => a.1.partial_cmp(&b.1).unwrap(),
        ord => ord,
    });

    let total_winnings = scored
        .iter()
        .enumerate()
        .fold(0, |total, (index, hand)| total + hand.2 * (1 + index as u32));

    Ok(total_winnings)
}

fn get_hand_type(hand: &str) -> HandType {
    let mut duplicates: Vec<u32>  = hand
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
        .into_iter()
        .map(|(_, v)| v)
        .collect();

    duplicates.sort_unstable();
    duplicates.reverse();

    let mut hand_type = HandType::HighCard;
    for i in duplicates {
        if i == 5 {
            hand_type = HandType::FiveOfAKind;
            break;
        }
        else if i == 4 {
            hand_type = HandType::FourOfAKind;
            break;
        }
        else if i == 3 {
            hand_type = HandType::ThreeOfAKind;
        }
        else if i == 2 {
            if hand_type == HandType::ThreeOfAKind {
                hand_type = HandType::FullHouse;
                break;
            }
            else if hand_type == HandType::OnePair {
                hand_type = HandType::TwoPairs;
                break;
            }
            hand_type = HandType::OnePair;
        }
    }

    hand_type
}

fn get_hand_type_with_joker(hand: &str) -> HandType {
    let mut duplicates: Vec<(char, u32)>  = hand
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            if c != 'J' {
                *map.entry(c).or_insert(0) += 1;
            }
            map
        })
        .into_iter()
        .collect();

    duplicates.sort_unstable_by_key(|i| i.1);
    let better_hand = hand.replace('J', duplicates.last().unwrap_or(&('A', 0)).0.to_string().as_str());
    get_hand_type(&better_hand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_7_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_one(&parsed);
        assert_eq!(result, Ok(6440));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_7_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_two(&parsed);
        assert_eq!(result, Ok(5905));
    }
}
