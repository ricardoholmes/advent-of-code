use std::collections::HashSet;

type Parsed = Vec<u32>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(
        input_raw.lines()
                 .map(|line| line.chars()
                                       .map(|c| c.to_digit(10)
                                                       .unwrap()
                                            )
                                       .collect()
                    )
                 .collect()
    )
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut count = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != 0 {
                continue;
            }

            let mut destinations = HashSet::new();
            trailhead_score(input, &mut destinations, x, y);
            count += destinations.len();
        }
    }
    Ok(count)
}

fn trailhead_score(map: &[Parsed], destinations: &mut HashSet<(usize,usize)>, x: usize, y: usize) {
    let cell = map[y][x];
    if cell == 9 {
        destinations.insert((x,y));
        return;
    }

    if x > 0 && map[y][x-1] == cell + 1 {
        trailhead_score(map, destinations, x-1, y);
    }

    if x + 1 < map[y].len() && map[y][x+1] == cell + 1 {
        trailhead_score(map, destinations, x+1, y);
    }

    if y > 0 && map[y-1][x] == cell + 1 {
        trailhead_score(map, destinations, x, y-1);
    }

    if y + 1 < map.len() && map[y+1][x] == cell + 1 {
        trailhead_score(map, destinations, x, y+1);
    }
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    let mut count = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != 0 {
                continue;
            }

            count += distinct_path_count(input, cell, x, y);
        }
    }
    Ok(count)
}

fn distinct_path_count(map: &[Parsed], cell: u32, x: usize, y: usize) -> usize {
    if cell == 9 {
        return 1;
    }

    let mut score = 0;

    if x > 0 && map[y][x-1] == cell + 1 {
        score += distinct_path_count(map, map[y][x-1], x-1, y);
    }

    if x + 1 < map[y].len() && map[y][x+1] == cell + 1 {
        score += distinct_path_count(map, map[y][x+1], x+1, y);
    }

    if y > 0 && map[y-1][x] == cell + 1 {
        score += distinct_path_count(map, map[y-1][x], x, y-1);
    }

    if y + 1 < map.len() && map[y+1][x] == cell + 1 {
        score += distinct_path_count(map, map[y+1][x], x, y+1);
    }

    score
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_10_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(36));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_10_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(81));
    }
}
