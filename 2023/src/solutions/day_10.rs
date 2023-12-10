use std::collections::HashMap;

type Coord = (usize, usize);

pub fn parse(input_raw: &str) -> Result<(Coord, HashMap<Coord, (Coord, Coord)>), String> {
    let lines = input_raw
        .lines();

    let mut map = HashMap::new();
    let mut start = (0, 0);

    for (y, line) in lines.clone().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                let mut connections: Vec<Coord> = vec![];
                if x > 0 && ['-', 'F', 'L'].contains(&line.chars().nth(x - 1).unwrap_or('.')) {
                    connections.push((x - 1, y));
                }
                if ['-', '7', 'J'].contains(&line.chars().nth(x + 1).unwrap_or('.')) {
                    connections.push((x + 1, y));
                }
                if y > 0 && ['|', 'F', '7'].contains(&lines.clone().nth(y - 1).unwrap_or("").chars().nth(x).unwrap_or('.')) {
                    connections.push((x, y - 1));
                }
                if ['|', 'L', 'J'].contains(&lines.clone().nth(y + 1).unwrap_or("").chars().nth(x).unwrap_or('.')) {
                    connections.push((x, y + 1));
                }

                if connections.len() != 2 {
                    panic!();
                }
                map.insert((x, y), (connections[0], connections[1]));
                start = (x, y);
            }
            else if c != '.' {
                map.insert((x, y), match c {
                    '-' => ((x.max(1) - 1, y), (x + 1, y)),
                    '|' => ((x, y.max(1) - 1), (x, y + 1)),
                    'F' => ((x, y + 1), (x + 1, y)),
                    '7' => ((x, y + 1), (x.max(1) - 1, y)),
                    'L' => ((x + 1, y), (x, y.max(1) - 1)),
                    'J' => ((x.max(1) - 1, y), (x, y.max(1) - 1)),
                    _ => panic!(),
                });
            }
        }
    }

    Ok((start, map))
}

pub fn part_one(input: &(Coord, HashMap<Coord, (Coord, Coord)>)) -> Result<u32, String> {
    let (start, map) = input;

    let mut loop_length = 1;
    let mut prev_node = *start;
    let mut node = map.get(start).unwrap().0;
    while node != *start {
        let connections = map.get(&node).unwrap();
        if prev_node == connections.0 {
            prev_node = node;
            node = connections.1;
        }
        else {
            prev_node = node;
            node = connections.0;
        }
        loop_length += 1;
    }

    Ok(loop_length / 2)
}

pub fn part_two(input: &(Coord, HashMap<Coord, (Coord, Coord)>)) -> Result<u32, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let example = include_str!("../../examples/day_10_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(4));
    }

    #[test]
    fn test_part1_2() {
        let example = include_str!("../../examples/day_10_2.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(8));
    }

    #[test]
    fn test_part2() {
        let example: &str = include_str!("../../examples/day_10_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(0));
    }
}
