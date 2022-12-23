use std::collections::{HashSet, HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    x: i32,
    y: i32
}

impl Cell {
    // returns true if there are any cells in W, NW, or SW adjacent positions
    fn check_surrounding(&self, map: &HashSet<Cell>) -> bool {
        !(self.check_up(map) && self.check_down(map) && self.check_left(map) && self.check_right(map))
    }

    // returns true if there are no cells in N, NW, or NE adjacent positions
    fn check_up(&self, map: &HashSet<Cell>) -> bool {
        for i in -1..=1 {
            if map.contains(&Cell { x: self.x + i, y: self.y - 1 }) {
                return false;
            }
        }
        true
    }

    // returns true if there are no cells in S, SW, or SE adjacent positions
    fn check_down(&self, map: &HashSet<Cell>) -> bool {
        for i in -1..=1 {
            if map.contains(&Cell { x: self.x + i, y: self.y + 1 }) {
                return false;
            }
        }
        true
    }

    // returns true if there are no cells in W, NW, or SW adjacent positions
    fn check_left(&self, map: &HashSet<Cell>) -> bool {
        for i in -1..=1 {
            if map.contains(&Cell { x: self.x - 1, y: self.y + i }) {
                return false;
            }
        }
        true
    }

    // returns true if there are no cells in E, NE, or SE adjacent positions
    fn check_right(&self, map: &HashSet<Cell>) -> bool {
        for i in -1..=1 {
            if map.contains(&Cell { x: self.x + 1, y: self.y + i }) {
                return false;
            }
        }
        true
    }
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_23.txt");

    let mut map: HashSet<Cell> = HashSet::new();
    
    let input: Vec<&str> = input_str.lines().collect();
    for (row_index, line) in input.iter().enumerate() {
        for (col_index, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(Cell {
                    x: col_index as i32,
                    y: row_index as i32
                });
            }
        }
    }

    part_one(&map);
}

fn part_one(map: &HashSet<Cell>) {
    let mut map = map.clone();

    for round in 0..10 {
        let mut proposed_cells: HashMap<Cell, Cell> = HashMap::new();
        for i in &map {
            if i.check_surrounding(&map) {
                match round % 4 {
                    0 => if i.check_up(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x, y: i.y - 1 });
                        } else if i.check_down(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x, y: i.y + 1 });
                        } else if i.check_left(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x - 1, y: i.y });
                        } else if i.check_right(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x + 1, y: i.y });
                        },

                    1 => if i.check_down(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x, y: i.y + 1 });
                        }
                        else if i.check_left(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x - 1, y: i.y });
                        }
                        else if i.check_right(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x + 1, y: i.y });
                        }
                        else if i.check_up(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x, y: i.y - 1 });
                        },

                    2 => if i.check_left(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x - 1, y: i.y });
                        }
                        else if i.check_right(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x + 1, y: i.y });
                        }
                        else if i.check_up(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x, y: i.y - 1 });
                        }
                        else if i.check_down(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x, y: i.y + 1 });
                        },

                    3 => if i.check_right(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x + 1, y: i.y });
                        }
                        else if i.check_up(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x, y: i.y - 1 });
                        }
                        else if i.check_down(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x, y: i.y + 1 });
                        }
                        else if i.check_left(&map) {
                            proposed_cells.insert(*i, Cell { x: i.x - 1, y: i.y });
                        },

                    _ => panic!("maths broke oopsie"),
                }
            }
        }
        for proposition in &proposed_cells {
            if proposed_cells.values().filter(|&cell| cell == proposition.1).count() == 1 {
                map.remove(proposition.0);
                map.insert(*proposition.1);
            }
        }
    }

    let mut highest_x = map.iter().next().unwrap().x;
    let mut highest_y = map.iter().next().unwrap().y;
    let mut lowest_x = highest_x;
    let mut lowest_y = highest_y;
    for cell in &map {
        highest_x = if cell.x > highest_x { cell.x } else { highest_x };
        highest_y = if cell.y > highest_y { cell.y } else { highest_y };
        lowest_x = if cell.x < lowest_x { cell.x } else { lowest_x };
        lowest_y = if cell.y < lowest_y { cell.y } else { lowest_y };
    }
    highest_x += 1;
    highest_y += 1;

    println!("Part one: {}", ((highest_x - lowest_x) * (highest_y - lowest_y)) - map.len() as i32);
}
