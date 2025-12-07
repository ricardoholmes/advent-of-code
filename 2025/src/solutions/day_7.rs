use std::collections::{HashSet, VecDeque};

// x start, list of splitter y positions per column, max depth
type Parsed = (usize, Vec<Vec<usize>>, usize);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut lines = input_raw.lines();

    let first_line = lines.next().unwrap();
    let start_x = first_line.find('S').unwrap();

    let mut splitters = vec![vec![]; first_line.len()];
    let mut y = 1;
    for line in lines {
        for (x, c) in line.char_indices() {
            if c == '^' {
                splitters[x].push(y);
            }
        }
        y += 1;
    }

    Ok((start_x, splitters, input_raw.lines().count()))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (start_x, splitters, _) = input;

    let mut out = 0;
    let mut beams = VecDeque::new();
    beams.push_front((*start_x, 0));
    let mut visited = HashSet::new();
    let mut visited_splitter = HashSet::new();

    while !beams.is_empty() {
        let (x, y) = beams.pop_front().unwrap();
        for &splitter_y in &splitters[x] {
            if splitter_y > y {
                if !visited_splitter.insert((x, splitter_y)) {
                    break;
                }

                let left = (x-1, splitter_y);
                if visited.insert(left) {
                    beams.push_back(left);
                }

                let right = (x+1, splitter_y);
                if visited.insert(right) {
                    beams.push_back(right);
                }

                out += 1;
                break;
            }
        }
    }

    Ok(out)
}

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    let (start_x, splitters, max_depth) = input;
    let width = splitters.len();

    let mut counts = vec![0; width];
    counts[*start_x] = 1;
    for depth in 1..*max_depth {
        let mut next_counts = counts.clone();
        for x in 0..width {
            for &splitter_y in &splitters[x] {
                if splitter_y == depth {
                    next_counts[x] = 0;
                    next_counts[x-1] += counts[x];
                    next_counts[x+1] += counts[x];
                    break;
                }
            }
        }
        counts = next_counts;
    }

    Ok(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_7_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(21));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_7_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(40));
    }
}
