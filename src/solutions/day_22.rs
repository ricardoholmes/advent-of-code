use std::collections::HashSet;

enum Movement {
    Direction(char),
    Magnitude(i32),
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_22.txt");

    let input: Vec<&str> = input_str
        .split("\n\n")
        .collect();

    let map: Vec<Vec<char>> = input[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut path: Vec<Movement> = vec![];
    let mut path_iter = input[1].lines().next().unwrap().chars();
    let mut num = String::new();
    while let Some(next_char) = path_iter.next() {
        if next_char.is_ascii_digit() {
            num.push(next_char);
        }
        else {
            if !num.is_empty() {
                path.push(Movement::Magnitude(num.parse().unwrap()));
                num.clear();
            }
            path.push(Movement::Direction(next_char));
        }
    }
    if !num.is_empty() {
        path.push(Movement::Magnitude(num.parse().unwrap()));
        num.clear();
    }

    part_one(&map, &path)
}

fn part_one(map: &Vec<Vec<char>>, path: &Vec<Movement>) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let (mut x, mut y) = (map[0].iter().position(|&c| c == '.').unwrap(), 0);
    visited.insert((x, y));

    let mut facing: i32 = 0;
    for step in path {
        match step {
            Movement::Direction(dir) => facing = match dir {
                'L' => (facing - 1).rem_euclid(4),
                'R' => (facing + 1).rem_euclid(4),
                _ => panic!()
            },
            Movement::Magnitude(magnitude) => for _ in 0..*magnitude {
                let mut next_x: i32 = x as i32;
                let mut next_y: i32 = y as i32;
                match facing {
                    0 => next_x += 1,
                    1 => next_y += 1,
                    2 => next_x -= 1,
                    3 => next_y -= 1,
                    _ => panic!()
                };
                if next_x < 0 || (facing == 2 && map[y][next_x as usize] == ' ') {
                    while next_x < map[y].len() as i32 - 1 && map[y][(next_x + 1) as usize] != ' ' {
                        next_x += 1;
                    }
                }
                if next_x >= map[y].len() as i32 || (facing == 0 && map[y][next_x as usize] == ' ') {
                    while next_x > 0 && map[y][(next_x - 1) as usize] != ' ' {
                        next_x -= 1;
                    }
                }
                if next_y < 0 || (facing == 3 && (map[next_y as usize].len() <= x || map[next_y as usize][x] == ' ')) {
                    while next_y < map.len() as i32 - 1 && map[(next_y + 1) as usize].len() > x && map[(next_y + 1) as usize][x] != ' ' {
                        next_y += 1;
                    }
                }
                if next_y >= map.len() as i32 || (facing == 1 && (map[next_y as usize].len() <= x || map[next_y as usize][x] == ' ')) {
                    while next_y > 0 && map[(next_y - 1) as usize].len() > x && map[(next_y - 1) as usize][x] != ' ' {
                        next_y -= 1;
                    }
                }

                if map[next_y as usize][next_x as usize] == '#' {
                    next_x = x as i32;
                    next_y = y as i32;
                }

                x = next_x as usize;
                y = next_y as usize;
                visited.insert((x, y));
            }
        };
    }

    println!("Part one: {}", 1000*(y + 1) + 4*(x + 1) + facing as usize);
}
