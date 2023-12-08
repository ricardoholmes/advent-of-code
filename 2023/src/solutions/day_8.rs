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

pub fn part_two(input: &(Vec<char>, HashMap<&str, (&str, &str)>)) -> Result<u64, String> {
    let (instructions, network) = input.clone();
    
    let mut nodes: Vec<&str> = network
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| *key)
        .collect();

    let mut completion_times = vec![];
    for node in nodes.iter_mut() {
        let mut instruction_index = 0;
        let mut steps: u64 = 0;
        while !node.ends_with("Z") {
            let instruction = instructions[instruction_index];
            *node = match instruction {
                'L' => network.get(node).unwrap().0,
                'R' => network.get(node).unwrap().1,
                _ => return Err(format!("Invalid character '{instruction}' in instructions")),
            };
            steps += 1;
            instruction_index = (instruction_index + 1) % instructions.len();
        }

        completion_times.push(steps);
    }

    let mut complete_time = 1;
    for steps in completion_times {
        complete_time = lcm(complete_time, steps);
    }

    Ok(complete_time)
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    }
    else {
        gcd(b, a % b)
    }
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
