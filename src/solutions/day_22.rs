#[derive(Debug)]
enum Movement {
    Direction(char),
    Magnitude(i32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn turn_left(&mut self) {
        const DIRECTIONS: [Facing; 4] = [Facing::Right, Facing::Down, Facing::Left, Facing::Up];
        let directions_index = (DIRECTIONS.iter().position(|i| i == self).unwrap() as i32 - 1).rem_euclid(4) as usize;
        *self = DIRECTIONS[directions_index];
    }

    fn turn_right(&mut self) {
        const DIRECTIONS: [Facing; 4] = [Facing::Right, Facing::Down, Facing::Left, Facing::Up];
        let directions_index = (DIRECTIONS.iter().position(|i| i == self).unwrap() as i32 + 1).rem_euclid(4) as usize;
        *self = DIRECTIONS[directions_index];
    }

    fn value(&self) -> usize {
        const DIRECTIONS: [Facing; 4] = [Facing::Right, Facing::Down, Facing::Left, Facing::Up];
        DIRECTIONS.iter().position(|i| i == self).unwrap()
    }
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
    let path_iter = input[1].lines().next().unwrap().chars();
    let mut num = String::new();
    for next_char in path_iter {
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

    part_one(&map, &path);
    part_two(&map, &path);
}

fn part_one(map: &[Vec<char>], path: &[Movement]) {
    let (mut x, mut y) = (map[0].iter().position(|&c| c == '.').unwrap(), 0);

    let mut facing = Facing::Right;
    for step in path {
        match step {
            Movement::Direction(dir) => match dir {
                'L' => facing.turn_left(),
                'R' => facing.turn_right(),
                _ => panic!()
            },
            Movement::Magnitude(magnitude) => for _ in 0..*magnitude {
                let mut next_x: i32 = x as i32;
                let mut next_y: i32 = y as i32;
                match facing {
                    Facing::Right => next_x += 1,
                    Facing::Down => next_y += 1,
                    Facing::Left => next_x -= 1,
                    Facing::Up => next_y -= 1,
                };
                if next_x < 0 || (facing == Facing::Left && map[y][next_x as usize] == ' ') {
                    while next_x < map[y].len() as i32 - 1 && map[y][(next_x + 1) as usize] != ' ' {
                        next_x += 1;
                    }
                }
                else if next_x >= map[y].len() as i32 || (facing == Facing::Right && map[y][next_x as usize] == ' ') {
                    while next_x > 0 && map[y][(next_x - 1) as usize] != ' ' {
                        next_x -= 1;
                    }
                }
                else if next_y < 0 || (facing == Facing::Up && (map[next_y as usize].len() <= x || map[next_y as usize][x] == ' ')) {
                    while next_y < map.len() as i32 - 1 && map[(next_y + 1) as usize].len() > x && map[(next_y + 1) as usize][x] != ' ' {
                        next_y += 1;
                    }
                }
                else if next_y >= map.len() as i32 || (facing == Facing::Down && (map[next_y as usize].len() <= x || map[next_y as usize][x] == ' ')) {
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
            }
        };
    }

    println!("Part one: {}", 1000*(y + 1) + 4*(x + 1) + facing.value());
}

fn part_two(map: &[Vec<char>], path: &[Movement]) {
    let (mut x, mut y) = (map[0].iter().position(|&c| c == '.').unwrap()+1, 1);

    let mut facing = Facing::Right;
    for step in path {
        match step {
            Movement::Direction(dir) => match dir {
                'L' => facing.turn_left(),
                'R' => facing.turn_right(),
                _ => panic!()
            },
            Movement::Magnitude(magnitude) => for _ in 0..*magnitude {
                let mut next_x = x;
                let mut next_y = y;
                let mut next_facing = facing;
                match facing {
                    Facing::Right => next_x += 1,
                    Facing::Down => next_y += 1,
                    Facing::Left => next_x -= 1,
                    Facing::Up => next_y -= 1,
                }

                // hard coded wrapping because doing it properly is too hard

                // wrap right
                if next_x == 151 && (1..=50).contains(&y) {
                    next_x = 100;
                    next_y = 151 - y;
                    next_facing = Facing::Left;
                }
                else if next_x == 101 && (51..=100).contains(&y) {
                    next_x = y + 50;
                    next_y = 50;
                    next_facing = Facing::Up;
                }
                else if next_x == 101 && (101..=150).contains(&y){
                    next_x = 150;
                    next_y = 151 - y;
                    next_facing = Facing::Left;
                }
                else if next_x == 51 && (151..=200).contains(&y) {
                    next_x = y - 100;
                    next_y = 150;
                    next_facing = Facing::Up;
                }

                // wrap left
                else if next_x == 50 && (1..=50).contains(&y) {
                    next_x = 1;
                    next_y = 151 - y;
                    next_facing = Facing::Right;
                }
                else if next_x == 50 && (51..=100).contains(&y) {
                    next_x = y - 50;
                    next_y = 101;
                    next_facing = Facing::Down;
                }
                else if next_x == 0 && (101..=150).contains(&y) {
                    next_x = 51;
                    next_y = 151 - y;
                    next_facing = Facing::Right;
                }
                else if next_x == 0 && (151..=200).contains(&y) {
                    next_x = y - 100;
                    next_y = 1;
                    next_facing = Facing::Down;
                }

                // wrap up
                else if next_y == 100 && (1..=50).contains(&x) {
                    next_x = 51;
                    next_y = x + 50;
                    next_facing = Facing::Right;
                }
                else if next_y == 0 && (51..=100).contains(&x) {
                    next_x = 1;
                    next_y = x + 100;
                    next_facing = Facing::Right;
                }
                else if next_y == 0 && (101..=150).contains(&x) {
                    next_x = x - 100;
                    next_y = 200;
                    next_facing = Facing::Up;
                }
                
                // wrap down
                else if next_y == 201 && (1..=50).contains(&x) {
                    next_x = x + 100;
                    next_y = 1;
                    next_facing = Facing::Down;
                }
                else if next_y == 151 && (51..=100).contains(&x) {
                    next_x = 50;
                    next_y = x + 100;
                    next_facing = Facing::Left;
                }
                else if next_y == 51 && (101..=150).contains(&x) {
                    next_x = 100;
                    next_y = x - 50;
                    next_facing = Facing::Left;
                }

                if map[next_y - 1][next_x - 1] != '#' {
                    x = next_x;
                    y = next_y;
                    facing = next_facing;
                }
            }
        }
    }

    println!("Part two: {}", 1000*y + 4*x + facing.value());
}
