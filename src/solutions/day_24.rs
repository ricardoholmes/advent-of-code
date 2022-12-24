use std::collections::{HashSet, HashMap};

#[derive(Clone)]
struct Blizzard {
    pos: (usize, usize),
    direction: Direction,
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_24.txt");

    let mut map: Vec<Vec<char>> = input_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut blizzards: Vec<Blizzard> = vec![];
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if ['<', '>', '^', 'v'].contains(&map[row][column]) {
                blizzards.push(Blizzard {
                    pos: (column, row),
                    direction: match map[row][column] {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => panic!() // unreachable
                    }
                });
                map[row][column] = '.';
            }
        }
    }

    let mut blizzards_positions: Vec<HashSet<(usize, usize)>> = vec![];
    loop {
        // move blizzards
        let mut blizzards_set: HashSet<(usize, usize)> = HashSet::new();
        for i in &mut blizzards {
            match i.direction {
                Direction::Up => {
                    i.pos.1 -= 1;
                    if i.pos.1 == 0 {
                        i.pos.1 = map.len() - 2;
                    }
                },
                Direction::Down => {
                    i.pos.1 += 1;
                    if i.pos.1 == map.len() - 1 {
                        i.pos.1 = 1;
                    }
                },
                Direction::Left => {
                    i.pos.0 -= 1;
                    if i.pos.0 == 0 {
                        i.pos.0 = map[0].len() - 2;
                    }
                },
                Direction::Right => {
                    i.pos.0 += 1;
                    if i.pos.0 == map[0].len() - 1 {
                        i.pos.0 = 1;
                    }
                },
            }
            blizzards_set.insert(i.pos);
        }
        if blizzards_positions.contains(&blizzards_set) {
            break;
        }
        blizzards_positions.push(blizzards_set);
    }

    part_one(&map, &blizzards_positions);
    part_two(&map, &blizzards_positions);
}

fn part_one(map: &Vec<Vec<char>>, blizzards: &Vec<HashSet<(usize, usize)>>) {
    let start: (usize, usize) = (map[0].iter().position(|&c| c == '.').unwrap(), 0);
    let end: (usize, usize) = (map[map.len()-1].iter().position(|&c| c == '.').unwrap(), map.len()-1);
    let least_steps = find_path(map, blizzards, start, end, 0);
    println!("Part one: {least_steps}");
}

fn part_two(map: &Vec<Vec<char>>, blizzards: &Vec<HashSet<(usize, usize)>>) {
    let start: (usize, usize) = (map[0].iter().position(|&c| c == '.').unwrap(), 0);
    let end: (usize, usize) = (map[map.len()-1].iter().position(|&c| c == '.').unwrap(), map.len()-1);

    let mut least_steps = find_path(map, blizzards, start, end, 0);
    least_steps = find_path(map, blizzards, end, start, least_steps);
    least_steps = find_path(map, blizzards, start, end, least_steps);

    println!("Part two: {least_steps}");
}

fn find_path(map: &Vec<Vec<char>>, blizzards: &Vec<HashSet<(usize, usize)>>, start: (usize, usize), end: (usize, usize), start_step: usize) -> usize {
    let mut queue: Vec<((usize, usize), usize)> = vec![(start, start_step)];
    let mut visited: HashMap<((usize, usize), usize), usize> = HashMap::new();
    let mut least_steps = usize::MAX;
    while queue.len() > 0 {
        let (pos, steps) = queue.pop().unwrap();

        let min_steps_at_end = end.0.abs_diff(pos.0) + end.1.abs_diff(pos.1) + steps;
        if min_steps_at_end >= least_steps {
            continue;
        }
        if pos == end {
            least_steps = steps;
            continue;
        }

        let blizzards_set = blizzards[steps as usize % blizzards.len()].clone();

        let steps = steps + 1;
        let mut new_positions: Vec<(usize, usize)> = vec![];
        if pos.1 > 0 {
            new_positions.push((pos.0, pos.1 - 1));
        }
        if pos.1 + 1 < map.len() {
            new_positions.push((pos.0, pos.1 + 1));
        }
        if min_steps_at_end < least_steps {
            new_positions.push((if end.0 > start.0 { pos.0 - 1 } else { pos.0 + 1 }, pos.1));

            if !blizzards_set.contains(&pos) && (!visited.contains_key(&(pos, steps % blizzards.len())) || visited.get(&(pos, steps % blizzards.len())).unwrap() > &steps) {
                queue.push((pos, steps));
                visited.insert((pos, steps % blizzards.len()), steps);
            }
        }
        new_positions.push((if end.0 > start.0 { pos.0 + 1 } else { pos.0 - 1 }, pos.1));

        for new_pos in new_positions {
            if new_pos != start && map[new_pos.1][new_pos.0] == '.' && !blizzards_set.contains(&new_pos) && (!visited.contains_key(&(new_pos, steps % blizzards.len())) || visited.get(&(new_pos, steps % blizzards.len())).unwrap() > &steps) {
                queue.push((new_pos, steps));
                visited.insert((new_pos, steps % blizzards.len()), steps);
            }
        }
    }

    least_steps
}
