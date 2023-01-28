use std::fs;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
struct Node {
    weight: u32, // individual
    total_weight: u32, // total needed to get this node
    coord: (usize, usize), // (X, y)
}

pub fn run() {
    let input = fs::read_to_string("inputs/input_15.txt")
        .expect("Failed to read file");
        
    // split input into vector of vector of ints
    // only needs 8 bits per int since each one is a single digit (in base 10)
    let input: Vec<Vec<u32>> = input
        .split("\n")
        .map(|s| s.chars().map(|x| x.to_string().parse().unwrap()).collect::<Vec<u32>>())
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<Vec<u32>>) {
    let mut map: Vec<Vec<Node>> = vec!();
    for i in 0..input.len() {
        map.push(vec!());
        for j in 0..input[0].len() {
            map[i].push(Node {
                weight: input[i][j],
                total_weight: u32::MAX,
                coord: (j, i),
            });
        }
    };

    println!("Part 1: {}", shortest_path(map));
}

fn part_two(input: &Vec<Vec<u32>>) {
    let mut start_tile: Vec<Vec<Node>> = vec!();
    let mut map: Vec<Vec<Node>> = vec!();
    for i in 0..input.len() {
        start_tile.push(vec!());
        map.push(vec!());
        for j in 0..input[0].len() {
            start_tile[i].push(Node {
                weight: input[i][j],
                total_weight: u32::MAX,
                coord: (j, i),
            });
            map[i].push(start_tile[i][j]);
        }
    }

    // complete rest of map
    for i in 1..5usize {
        // fill next tile to first row of tiles
        for row in 0..start_tile.len() {
            for col in 0..start_tile[row].len() {
                let mut cell = start_tile[row][col];
                cell.weight = if cell.weight + i as u32 <= 9 { cell.weight + i as u32 } else { (cell.weight + i as u32) - 9 };
                cell.coord = (cell.coord.0 + (i * start_tile[0].len()), cell.coord.1);
                map[row].push(cell);
            }
        }

        for _row in 0..start_tile.len() {
            map.push(vec!());
        }

        // fill ith row 
        for j in 0..5usize {
            for row in 0..start_tile.len() {
                for col in 0..start_tile[row].len() {
                    let mut cell = start_tile[row][col];
                    cell.weight = if cell.weight + (i + j) as u32 <= 9 { cell.weight + (i + j) as u32 } else { (cell.weight + (i + j) as u32) - 9 };
                    cell.coord = (cell.coord.0 + (j * start_tile[0].len()), cell.coord.1 + ((i as usize) * start_tile[0].len()));
                    map[row + ((i as usize) * start_tile.len())].push(cell);
                }
            }
        }
    }

    println!("Part 2: {}", shortest_path(map));
}

fn shortest_path(mut map: Vec<Vec<Node>>) -> u32 {
    map[0][0].weight = 0;
    map[0][0].total_weight = 0;

    let end_coord = map[map.len()-1][map[0].len()-1].coord;

    let mut finished_nodes: HashSet<Node> = HashSet::new();
    let mut queue: Vec<Node> = vec!(map[0][0]);

    // let mut nodes_examined = 0;
    loop {
        let node: Node = queue[0];
        finished_nodes.insert(node);
        queue.remove(0);
        // nodes_examined += 1;

        if node.coord == end_coord {
            // println!("{} nodes examined", nodes_examined);
            return node.total_weight;
        }

        let (x, y) = node.coord;

        if x > 0 && !finished_nodes.contains(&map[y][x-1]) {
            let old_weight = map[y][x-1].total_weight;
            let new_weight = node.total_weight + map[y][x-1].weight;
            if new_weight < old_weight {
                if let Some(old_index) = queue.iter().position(|e| e == &map[y][x-1]) {
                    queue.remove(old_index);
                }

                map[y][x-1].total_weight = new_weight;

                queue = insert(queue, map[y][x-1]);
            }
        }

        if x < end_coord.0 && !finished_nodes.contains(&map[y][x+1]) {
            let old_weight = map[y][x+1].total_weight;
            let new_weight = node.total_weight + map[y][x+1].weight;
            if new_weight < old_weight {
                if let Some(old_index) = queue.iter().position(|e| e == &map[y][x+1]) {
                    queue.remove(old_index);
                }

                map[y][x+1].total_weight = new_weight;

                queue = insert(queue, map[y][x+1]);
            }
        }

        if y > 0 && !finished_nodes.contains(&map[y-1][x]) {
            let old_weight = map[y-1][x].total_weight;
            let new_weight = node.total_weight + map[y-1][x].weight;
            if new_weight < old_weight {
                if let Some(old_index) = queue.iter().position(|e| e == &map[y-1][x]) {
                    queue.remove(old_index);
                }

                map[y-1][x].total_weight = new_weight;

                queue = insert(queue, map[y-1][x]);
            }
        }

        if y < end_coord.1 && !finished_nodes.contains(&map[y+1][x]) {
            let old_weight = map[y+1][x].total_weight;
            let new_weight = node.total_weight + map[y+1][x].weight;
            if new_weight < old_weight {
                if let Some(old_index) = queue.iter().position(|e| e == &map[y+1][x]) {
                    queue.remove(old_index);
                }

                map[y+1][x].total_weight = new_weight;

                queue = insert(queue, map[y+1][x]);
            }
        }
    }
}

// insert so that it remains sorted
fn insert(mut queue: Vec<Node>, value: Node) -> Vec<Node> {
    let mut inserted = false;
    let mut previous = 0;
    for i in 0..queue.len() {
        if previous < value.total_weight && queue[i].total_weight >= value.total_weight {
            queue.insert(i, value);
            inserted = true;
        }
        previous = queue[i].total_weight;
    }

    if !inserted {
        queue.push(value);
    }

    queue
}
