use std::collections::HashSet;
use std::collections::VecDeque;

pub fn run() {
    let input_str = include_str!("../../inputs/input_12.txt");

    // make input into list of sums of calories
    let input: Vec<Vec<char>> = input_str
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<Vec<char>>) {
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut paths: VecDeque<Vec<(usize, usize)>> = VecDeque::new();

    let mut start_node: (usize, usize) = (0, 0);
    'outer: for row in 0..input.len() {
        for column in 0..input[row].len() {
            if input[row][column] == 'S' {
                start_node = (row, column);
                break 'outer;
            }
        }
    }
    visited_nodes.insert(start_node);
    paths.push_back(vec!(start_node));

    'outer: loop {
        let path = paths[0].clone();
        let current_node = path.last().unwrap();
        let current_node_val = match input[current_node.0][current_node.1] {
            'S' => 'a',
            x @ _ => x,
        };

        for (row_offset, column_offset) in vec!((0, 1), (2, 1), (1, 0), (1, 2)) {
            if current_node.0 + row_offset == 0 || current_node.0 + row_offset-1 >= input.len() ||
                current_node.1 + column_offset == 0 || current_node.1 + column_offset-1 >= input[0].len()
            {
                continue;
            }

            let coord = (current_node.0 + row_offset-1, current_node.1 + column_offset-1);
            let cell = input[current_node.0 + row_offset-1][current_node.1 + column_offset-1];

            if !visited_nodes.contains(&coord) && (cell == current_node_val ||
                cell as u32 <= current_node_val as u32 + 1 ||
                (cell == 'E' && current_node_val >= 'y'))
            {
                visited_nodes.insert(coord);
                let mut new_path = path.clone();
                new_path.push(coord);
                paths.push_back(new_path);
                if cell == 'E' {
                    break 'outer;
                }
            }
        }
        paths.pop_front();
    }

    println!("Part one: {}", paths.pop_back().unwrap().len() - 1);
}

fn part_two(input: &Vec<Vec<char>>) {
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut paths: VecDeque<Vec<(usize, usize)>> = VecDeque::new();

    let mut start_node: (usize, usize) = (0, 0);
    'outer: for row in 0..input.len() {
        for column in 0..input[row].len() {
            if input[row][column] == 'E' {
                start_node = (row, column);
                break 'outer;
            }
        }
    }
    visited_nodes.insert(start_node);
    paths.push_back(vec!(start_node));

    'outer: loop {
        let path = paths[0].clone();
        let current_node = path.last().unwrap();
        let current_node_val = match input[current_node.0][current_node.1] {
            'E' => 'z',
            x @ _ => x,
        };

        for (row_offset, column_offset) in vec!((0, 1), (2, 1), (1, 0), (1, 2)) {
            if current_node.0 + row_offset == 0 || current_node.0 + row_offset-1 >= input.len() ||
                current_node.1 + column_offset == 0 || current_node.1 + column_offset-1 >= input[0].len()
            {
                continue;
            }

            let coord = (current_node.0 + row_offset-1, current_node.1 + column_offset-1);
            let cell = input[current_node.0 + row_offset-1][current_node.1 + column_offset-1];

            if !visited_nodes.contains(&coord) && (cell == current_node_val ||
                cell as u32 >= current_node_val as u32 - 1 ||
                (current_node_val == 'b' && cell == 'S'))
            {
                visited_nodes.insert(coord);
                let mut new_path = path.clone();
                new_path.push(coord);
                paths.push_back(new_path);
                if cell == 'a' || cell == 'S' {
                    break 'outer;
                }
            }
        }
        paths.pop_front();
    }

    println!("Part two: {}", paths.pop_back().unwrap().len() - 1);
}
