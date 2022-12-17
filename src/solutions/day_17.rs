use core::panic;
use std::collections::{HashSet, HashMap};

#[derive(Clone)]
struct Rock {
    shape: Vec<(i64, i64)>,
}

impl Rock {
    fn spawn(&self, highest_y: i64) -> Vec<(i64, i64)> {
        let mut positions = vec![];
        for i in &self.shape {
            positions.push((i.0, i.1 + highest_y + 4));
        }
        positions
    }

    fn move_left(&self, positions: &Vec<(i64, i64)>, map: &HashSet<(i64, i64)>) -> Vec<(i64, i64)> {
        let mut new_positions = vec![];
        for i in positions {
            let pos = (i.0 - 1, i.1);
            if pos.0 < -3 || map.contains(&pos) {
                return positions.to_vec();
            }
            new_positions.push(pos)
        }
        new_positions
    }

    fn move_right(&self, positions: &Vec<(i64, i64)>, map: &HashSet<(i64, i64)>) -> Vec<(i64, i64)>  {
        let mut new_positions = vec![];
        for i in positions {
            let pos = (i.0 + 1, i.1);
            if pos.0 > 3 || map.contains(&pos) {
                return positions.to_vec();
            }
            new_positions.push(pos)
        }
        new_positions
    }

    fn move_down(&self, positions: &Vec<(i64, i64)>, map: &HashSet<(i64, i64)>) -> Vec<(i64, i64)>  {
        let mut new_positions = vec![];
        for i in positions {
            let pos = (i.0, i.1 - 1);
            if pos.1 < 1 || map.contains(&pos) {
                return positions.to_vec();
            }
            new_positions.push(pos)
        }
        new_positions
    }
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_17.txt");

    let input: Vec<char> = input_str.lines().next().unwrap().chars().collect();
    let rocks: [Rock; 5] = [
        Rock { shape: vec![(-1, 0), (0, 0), (1, 0), (2, 0)] },
        Rock { shape: vec![(0, 0), (-1, 1), (0, 1), (1, 1), (0, 2)] },
        Rock { shape: vec![(-1, 0), (0, 0), (1, 0), (1, 1), (1, 2)] },
        Rock { shape: vec![(-1, 0), (-1, 1), (-1, 2), (-1, 3)] },
        Rock { shape: vec![(-1, 0), (0, 0), (-1, 1), (0, 1)] }
    ];

    part_one(&input, &rocks);
    part_two(&input, &rocks);
}

fn part_one(jet_pattern: &Vec<char>, rocks: &[Rock; 5]) {
    let mut amount_landed = 0;
    let mut jet_pattern_index = 0;
    let mut rock_index = 0;
    let mut rock = rocks.get(0).unwrap().clone();
    let mut highest_y = 0;
    let mut positions = rock.spawn(highest_y);
    let mut map: HashSet<(i64, i64)> = HashSet::new();

    while amount_landed < 2022 {
        positions = match jet_pattern[jet_pattern_index] {
            '<' => rock.move_left(&positions.clone(), &map),
            '>' => rock.move_right(&positions.clone(), &map),
            _ => panic!(),
        };

        let new_pos = rock.move_down(&positions, &map);
        if new_pos == positions {
            for i in positions {
                if i.1 > highest_y {
                    highest_y = i.1;
                }
                map.insert(i);
            }
            rock_index = (rock_index + 1) % 5;
            rock = rocks.get(rock_index).unwrap().clone();
            positions = rock.spawn(highest_y);
            amount_landed += 1;
        }
        else {
            positions = new_pos;
        }

        jet_pattern_index = (jet_pattern_index + 1) % jet_pattern.len();
    }

    println!("Part one: {highest_y}");
}

fn part_two(jet_pattern: &Vec<char>, rocks: &[Rock; 5]) {
    let mut amount_landed: i64 = 0;
    let mut jet_pattern_index = 0;
    let mut rock_index = 0;
    let mut rock = rocks.get(0).unwrap().clone();
    let mut highest_y = 0;
    let mut positions = rock.spawn(highest_y);
    let mut map: HashSet<(i64, i64)> = HashSet::new();
    let mut potential_cycles: HashMap<(usize, usize, Vec<u8>), (i64, i64)> = HashMap::new();
    // (jet_pattern_index, rock_index) : (highest_y, amount_landed)
    let mut cycle: (i64, i64) = (0, 0);
    // (height of cycle, landings per cycle)
    let mut cycle_found = false;

    loop {
        positions = match jet_pattern[jet_pattern_index] {
            '<' => rock.move_left(&positions.clone(), &map),
            '>' => rock.move_right(&positions.clone(), &map),
            _ => panic!(),
        };

        let new_pos = rock.move_down(&positions, &map);
        if new_pos == positions {
            for i in &positions {
                if i.1 > highest_y {
                    highest_y = i.1;
                }
                map.insert(*i);
            }
            amount_landed += 1;

            let bottom_row: Vec<u8> = (highest_y-30..=highest_y).map(|y|
                    (-3..=3).fold(0,
                        |total, x| (total << 1) + (map.contains(&(x, y)) as u8)
                    )
                ).collect();

            let indexes = (jet_pattern_index, rock_index, bottom_row);
            if !cycle_found && potential_cycles.contains_key(&indexes) {
                let cycle_info = *potential_cycles.get(&indexes).unwrap();
                cycle = (highest_y - cycle_info.0, amount_landed - cycle_info.1);
                cycle_found = true;
                if (1_000_000_000_000 - amount_landed) % cycle.1 == 0 {
                    break;
                }
            }
            else if cycle_found && (1_000_000_000_000 - amount_landed) % cycle.1 == 0 {
                break;
            }
            else {
                potential_cycles.insert(indexes, (highest_y, amount_landed));
            }

            rock_index = (rock_index + 1) % 5;
            rock = rocks.get(rock_index).unwrap().clone();
            positions = rock.spawn(highest_y);
        }
        else {
            positions = new_pos;
        }

        jet_pattern_index = (jet_pattern_index + 1) % jet_pattern.len();
    }

    let height = highest_y + cycle.0 * ((1_000_000_000_000 - amount_landed) / cycle.1);
    println!("Part two: {height}");
}
