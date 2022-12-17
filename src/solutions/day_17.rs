use core::panic;
use std::collections::HashSet;

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
