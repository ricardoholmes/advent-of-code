use std::collections::HashSet;

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
}

fn part_one(map: &Vec<Vec<char>>, blizzards: &Vec<HashSet<(usize, usize)>>) {
    let start: (usize, usize) = (map[0].iter().position(|&c| c == '.').unwrap(), 0);
    let destination: (usize, usize) = (map[map.len()-1].iter().position(|&c| c == '.').unwrap(), map.len()-1);

    let mut queue: Vec<((usize, usize), usize, usize)> = vec![(start, 0, 0)];
    let mut visited: HashSet<((usize, usize), usize)> = HashSet::new();
    let mut least_steps = usize::MAX;
    while queue.len() > 0 {
        let (pos, steps,  waiting_time) = queue.pop().unwrap();

        let min_steps_at_end = ((destination.0 - pos.0) + (destination.1 - pos.1)) + steps;
        if min_steps_at_end >= least_steps {
            continue;
        }
        if pos == destination {
            least_steps = steps;
            continue;
        }

        let blizzards_set = blizzards[steps as usize % blizzards.len()].clone();

        if pos.1 > 1 {
            let new_pos = (pos.0, pos.1 - 1);
            if map[new_pos.1][new_pos.0] == '.' && !blizzards_set.contains(&new_pos) && !visited.contains(&(new_pos, steps+1)) {
                queue.push((new_pos, steps+1, 0));
                visited.insert((new_pos, steps+1));
            }
        }
        if min_steps_at_end < least_steps {
            let new_pos = (pos.0 - 1, pos.1);
            if map[new_pos.1][new_pos.0] == '.' && !blizzards_set.contains(&new_pos) && !visited.contains(&(new_pos, steps+1)) {
                queue.push((new_pos, steps+1, 0));
                visited.insert((new_pos, steps+1));
            }

            if waiting_time + 1 < blizzards.len() && !blizzards_set.contains(&pos) && !visited.contains(&(pos, steps+1)) {
                queue.push((pos, steps+1, waiting_time + 1));
                visited.insert((pos, steps+1));
            }
        }
        for new_pos in [(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)] {
            if new_pos != start && map[new_pos.1][new_pos.0] == '.' && !blizzards_set.contains(&new_pos) && !visited.contains(&(new_pos, steps+1)) {
                queue.push((new_pos, steps+1, 0));
                visited.insert((new_pos, steps+1));
            }
        }
    }

    println!("Part one: {least_steps}");
}
