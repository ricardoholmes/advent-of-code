use std::fs;

#[derive(Clone, Copy, PartialEq)]
struct Node {
    weight: u32, // individual
    total_weight: u32, // total needed to get this node
    combined_heuristic: u32, // total weight + distance to end
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
}

fn part_one(input: &Vec<Vec<u32>>) {
    let mut map: Vec<Vec<Node>> = vec!();
    for i in 0..input.len() {
        map.push(vec!());
        for j in 0..input[0].len() {
            map[i].push(Node {
                weight: input[i][j],
                total_weight: u32::MAX,
                combined_heuristic: 0,
                coord: (j, i),
            });
        }
    }
    map[0][0].weight = 0;
    map[0][0].total_weight = 0;

    let end_coord = (input[0].len()-1, input.len()-1);

    let mut finished_nodes: Vec<Node> = vec!();
    let mut queue: Vec<Node> = vec!(map[0][0]);

    loop {
        let node: Node = queue[0];
        finished_nodes.push(node);
        queue.remove(0);
        
        // println!("{:?}, {}, {}", node.coord, node.total_weight, node.combined_heuristic);

        if node.coord == end_coord {
            break;
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

                let dist_from_end = (end_coord.0 - (x - 1)) + (end_coord.1 - y);
                map[y][x-1].combined_heuristic = new_weight + dist_from_end as u32;
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
                let dist_from_end = (end_coord.0 - (x + 1)) + (end_coord.1 - y);
                map[y][x+1].combined_heuristic = new_weight + dist_from_end as u32;
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
                let dist_from_end = (end_coord.0 - x) + (end_coord.1 - (y - 1));
                map[y-1][x].combined_heuristic = new_weight + dist_from_end as u32;
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
                let dist_from_end = (end_coord.0 - x) + (end_coord.1 - (y + 1));
                map[y+1][x].combined_heuristic = new_weight + dist_from_end as u32;
                queue = insert(queue, map[y+1][x]);
            }
        }
    }

    println!("Part 1: {}", finished_nodes[finished_nodes.len()-1].total_weight)
}

// insert so that it remains sorted
fn insert(mut queue: Vec<Node>, value: Node) -> Vec<Node> {
    let mut inserted = false;
    let mut previous = 0;
    for i in 0..queue.len() {
        if previous < value.combined_heuristic && queue[i].combined_heuristic >= value.combined_heuristic {
            queue.insert(i, value);
            inserted = true;
        }
        previous = queue[i].combined_heuristic;
    }

    if !inserted {
        queue.push(value);
    }

    queue
}
