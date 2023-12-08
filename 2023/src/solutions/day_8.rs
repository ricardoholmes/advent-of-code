use std::collections::HashMap;

pub fn parse(input_raw: &str) -> Result<(Vec<char>, HashMap<&str, (&str, &str)>), String> {
    let mut lines = input_raw.lines();

    let instructions = lines.next().unwrap().chars().collect();
    lines.next();

    let mut network = HashMap::new();

    for line in lines {
        let mut split = line.split(" = ");
        let node = split.next().unwrap();

        let mut connections = split
            .next()
            .unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ");

        network.insert(node, (connections.next().unwrap(), connections.next().unwrap()));
    }

    Ok((instructions, network))
}

pub fn part_one(input: &(Vec<char>, HashMap<&str, (&str, &str)>)) -> Result<u64, String> {
    let (instructions, network) = input.clone();

    let mut instruction_index = 0;
    let mut steps: u64 = 0;
    let mut node = "AAA";
    while node != "ZZZ" {
        let instruction = instructions[instruction_index];
        node = match instruction {
            'L' => network.get(node).unwrap().0,
            'R' => network.get(node).unwrap().1,
            _ => return Err(format!("Invalid character '{instruction}' in instructions")),
        };
        steps += 1;
        instruction_index = (instruction_index + 1) % instructions.len();
    }

    Ok(steps)
}

pub fn part_two(input: &(Vec<char>, HashMap<&str, (&str, &str)>)) -> Result<u128, String> {
    let (instructions, network) = input.clone();
    
    let mut nodes: Vec<(&str, HashMap<(&str, usize), u128>)> = network
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|node| (*node, HashMap::new()))
        .collect();

    let mut loops = vec![];
    let mut instruction_index;
    for node in nodes.iter_mut() {
        instruction_index = 0;
        let mut steps = 0;

        node.1.insert((node.0, instruction_index), steps);

        // (start_step_of_loop, length_of_loop, distance_to_ends)
        let mut node_loop: (u128, u128, u128) = (0, 0, 0);
        loop {
            steps += 1;

            let instruction = instructions[instruction_index];
            node.0 = match instruction {
                'L' => network.get(node.0).unwrap().0,
                'R' => network.get(node.0).unwrap().1,
                _ => return Err(format!("Invalid character '{instruction}' in instructions")),
            };

            if node_loop.1 == 0 {
                if let Some(start_of_loop) = node.1.get(&(node.0, instruction_index)) {
                    node_loop = (*start_of_loop, steps - start_of_loop, 0);
                }
                else {
                    node.1.insert((node.0, instruction_index), steps);
                }
            }
            else if node.0.ends_with("Z") {
                node_loop.2 = steps - node_loop.1;
            }

            instruction_index = (instruction_index + 1) % instructions.len();

            if node_loop.1 > 0 && steps - node_loop.0 > 2 * node_loop.1 {
                break;
            }
        }

        loops.push(node_loop);
    }

    let mut all_loop: (u128, u128, u128) = loops[0];
    for (n, i) in loops.iter().enumerate() {
        let i_length_big = i.1 * ((all_loop.2 / i.1) - 1);
        let mut i_end = i.2 + i_length_big;
        while all_loop.2 != i_end {
            if i_end < all_loop.2 {
                i_end += i.1;
            }
            else {
                all_loop.2 += all_loop.1;
            }
        }
        println!("{}", all_loop.2);

        all_loop.1 = lcm(all_loop.1, i.1);
        println!("{i:?} -> {all_loop:?} ({n} / {})", loops.len());
    }

    println!("{all_loop:?}");

    Ok(all_loop.2)
}

fn lcm(a: u128, b: u128) -> u128 {
    let mut gcd = 1;
    for i in 1..=a.min(b) {
        if a % i == 0 && b % i == 0 {
            gcd = i;
        }
    }

    a * b / gcd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let example = include_str!("../../examples/day_8_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(2));
    }

    #[test]
    fn test_part1_2() {
        let example = include_str!("../../examples/day_8_2.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(6));
    }

    #[test]
    fn test_part2() {
        let example: &str = include_str!("../../examples/day_8_3.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(6));
    }
}
