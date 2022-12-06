use std::collections::HashSet;

pub fn run() {
    let input: Vec<char> = include_str!("../../inputs/input_6.txt")
        .chars()
        .collect();

    let index = part_one(&input);
    part_two(&input, index)
}

fn part_one(input: &Vec<char>) -> usize {
    let input = input.clone();
    let mut index: usize = usize::MAX;
    for i in 0..(input.len() - 3) {
        let mut chars_set: HashSet<char> = HashSet::new();
        let section = input.iter().skip(i).take(4);

        for c in section {
            chars_set.insert(c.to_owned());
        }
        
        if chars_set.len() == 4 {
            println!("Part one: {}", i + 4);
            index = i + 4;
            break;
        }
    }

    return index;
}

fn part_two(input: &Vec<char>, index: usize) {
    let input = input.clone();
    for i in index..(input.len() - 13) {
        let mut chars_set: HashSet<char> = HashSet::new();
        let section = input.iter().skip(i).take(14);
        for c in section {
            chars_set.insert(c.to_owned());
        }
        
        if chars_set.len() == 14 {
            println!("Part two: {}", i + 14);
            break;
        }
    }
}
