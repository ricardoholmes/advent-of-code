type Parsed = ((Direction, i64), (Direction, i64));

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut parsed = vec![];

    for line in input_raw.lines() {
        let mut line_split = line.split_ascii_whitespace();

        let direction = match line_split.next() {
            Some("L") => Direction::Left,
            Some("R") => Direction::Right,
            Some("U") => Direction::Up,
            Some("D") => Direction::Down,
            _ => return Err(format!("Invalid input string")),
        };

        let amount = line_split.next().unwrap().parse().unwrap();

        let mut color = line_split.next().unwrap();
        color = &color[2..(color.len()-1)];

        let part2_direction = match &color[color.len() - 1..] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => return Err(format!("Invalid input string")),
        };

        let part2_amount = i64::from_str_radix(&color[..color.len() - 1], 16).unwrap();

        parsed.push(((direction, amount), (part2_direction, part2_amount)));
    }

    Ok(parsed)
}

pub fn part_one(input: &[Parsed]) -> Result<i64, String> {
    let mut trench_vertices = vec![];
    trench_vertices.push((0, 0));

    let mut x_bounds = (0, 0);
    let mut y_bounds = (0, 0);
    let mut position: (i64, i64) = (0, 0);
    for (i, _) in input {
        position = match i.0 {
            Direction::Left => (position.0 - i.1, position.1),
            Direction::Right => (position.0 + i.1, position.1),
            Direction::Up => (position.0, position.1 - i.1),
            Direction::Down => (position.0, position.1 + i.1),
        };

        if position.0 < x_bounds.0 {
            x_bounds.0 = position.0;
        }
        if position.0 > x_bounds.1 {
            x_bounds.1 = position.0;
        }
        
        if position.1 < y_bounds.0 {
            y_bounds.0 = position.1;
        }
        if position.1 > y_bounds.1 {
            y_bounds.1 = position.1;
        }

        trench_vertices.push(position);
    }

    let area = get_area(trench_vertices);

    Ok(area)
}

pub fn part_two(input: &[Parsed]) -> Result<i64, String> {
    let mut trench_vertices = vec![];
    trench_vertices.push((0, 0));

    let mut x_bounds = (0, 0);
    let mut y_bounds = (0, 0);
    let mut position: (i64, i64) = (0, 0);
    for (_, i) in input {
        position = match i.0 {
            Direction::Left => (position.0 - i.1, position.1),
            Direction::Right => (position.0 + i.1, position.1),
            Direction::Up => (position.0, position.1 - i.1),
            Direction::Down => (position.0, position.1 + i.1),
        };

        if position.0 < x_bounds.0 {
            x_bounds.0 = position.0;
        }
        if position.0 > x_bounds.1 {
            x_bounds.1 = position.0;
        }
        
        if position.1 < y_bounds.0 {
            y_bounds.0 = position.1;
        }
        if position.1 > y_bounds.1 {
            y_bounds.1 = position.1;
        }

        trench_vertices.push(position);
    }

    let area = get_area(trench_vertices);

    Ok(area)
}

fn get_area(trench_vertices: Vec<(i64, i64)>) -> i64 {
    let mut area = 0;
    let mut perimeter = 0;
    for (index, pos) in trench_vertices.iter().enumerate() {
        let next_pos = trench_vertices[(index + 1) % trench_vertices.len()];
        area += (next_pos.0 - pos.0) * (next_pos.1 + pos.1);

        // no diagonals so perimeter is trivial
        perimeter += ((next_pos.0 - pos.0) + (next_pos.1 - pos.1)).abs();
    }

    (area.abs() + perimeter) / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_18_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(62));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_18_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(952408144115));
    }
}
