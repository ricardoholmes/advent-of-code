use std::collections::{HashSet, HashMap};

type Point = (usize, usize);
type Line = Vec<char>;

pub fn parse(input_raw: &str) -> Result<Vec<Line>, String> {
    let lines = input_raw.lines().map(|line| line.chars().collect()).collect();

    Ok(lines)
}

pub fn part_one(grid: &[Line]) -> Result<usize, String> {
    let max_col = grid[0].len() - 1;
    let max_row = grid.len() - 1;

    let start = (grid[0].iter().position(|&ch| ch == '.').unwrap(), 0);
    let mut queue = vec![(start, HashSet::new())];
    let mut most_steps = 0;
    while !queue.is_empty() {
        let ((x, y), mut seen) = queue.pop().unwrap();
        let ch = grid[y][x];
        if ch == '#' {
            continue;
        }
        if !seen.insert((x, y))  {
            continue;
        }

        if ch == '.' && y == max_row {
            if seen.len() - 1 > most_steps {
                most_steps = seen.len() - 1;
            }
            continue;
        }

        if (ch == '^' || ch == '.') && y > 0 {
            queue.push(((x, y - 1), seen.clone()));
        }
        if (ch == '<' || ch == '.') && x > 0 {
            queue.push(((x - 1, y), seen.clone()));
        }
        if (ch == '>' || ch == '.') && x < max_col {
            queue.push(((x + 1, y), seen.clone()));
        }
        if (ch == 'v' || ch == '.') && y < max_row {
            queue.push(((x, y + 1), seen.clone()));
        }
    }

    Ok(most_steps)
}

pub fn part_two(grid: &[Line]) -> Result<usize, String> {
    let max_col = grid[0].len() - 1;
    let max_row = grid.len() - 1;

    let start = (grid[0].iter().position(|&ch| ch == '.').unwrap(), 0);
    let mut intersections: HashMap<Point, Vec<(Point, usize)>> = HashMap::new();
    let mut queue = vec![start];
    while !queue.is_empty() {
        let point = queue.pop().unwrap();
        if intersections.contains_key(&point) {
            continue;
        }

        let mut reachable_nodes = vec![];
        for x in point.0+1..=max_col {
            if grid[point.1][x] == '#' {
                break;
            }

            if (point.1 > 0 && grid[point.1 - 1][x] != '#') || (point.1 < max_row && grid[point.1 + 1][x] != '#') {
                reachable_nodes.push(((x, point.1), x - point.0));
                break;
            }
        }
        for x in (0..point.0).rev() {
            if grid[point.1][x] == '#' {
                break;
            }

            if (point.1 > 0 && grid[point.1 - 1][x] != '#') || (point.1 < max_row && grid[point.1 + 1][x] != '#') {
                reachable_nodes.push(((x, point.1), point.0 - x));
                break;
            }
        }
        for y in point.1+1..=max_row {
            if grid[y][point.0] == '#' {
                break;
            }

            if (point.0 > 0 && grid[y][point.0 - 1] != '#') || (point.0 < max_col && grid[y][point.0 + 1] != '#') || y == max_row {
                reachable_nodes.push(((point.0, y), y - point.1));
                break;
            }
        }
        for y in (0..point.1).rev() {
            if grid[y][point.0] == '#' {
                break;
            }

            if (point.0 > 0 && grid[y][point.0 - 1] != '#') || (point.0 < max_col && grid[y][point.0 + 1] != '#') || y == 0 {
                reachable_nodes.push(((point.0, y), point.1 - y));
                break;
            }
        }

        for (point, _) in &reachable_nodes {
            queue.push(*point);
        }
        intersections.insert(point, reachable_nodes.clone());
    }

    loop {
        let intersections_clone = intersections.clone();
        let maybe_redundancy = intersections_clone
            .iter()
            .find(|(_, destinations)| destinations.len() == 2);

        if let Some((redundant_node, destinations)) = maybe_redundancy {
            intersections.remove(redundant_node);
            let new_distance = destinations[0].1 + destinations[1].1;
            for i in 0..2 {
                let start = destinations[i].0;
                let end = destinations[1 - i].0;
                let dest_node = intersections.get_mut(&start).unwrap();
                if dest_node.contains(&destinations[1 - i]) {
                    continue;
                }

                for (node, distance) in dest_node {
                    if node == redundant_node {
                        *node = end;
                        *distance = new_distance;
                    }
                }
            }
        }
        else {
            break;
        }
    }

    let mut most_steps = 0;
    let mut queue = vec![(start, HashSet::new(), 0)];
    while !queue.is_empty() {
        let ((x, y), mut seen, steps_taken) = queue.pop().unwrap();
        if !seen.insert((x, y)) {
            continue;
        }

        if y == max_row {
            if steps_taken > most_steps {
                most_steps = steps_taken;
            }
            continue;
        }

        for (point, distance) in intersections.get(&(x, y)).unwrap() {
            queue.push((*point, seen.clone(), steps_taken + distance));
        }
    }

    Ok(most_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_23_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(94));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_23_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(154));
    }
}
