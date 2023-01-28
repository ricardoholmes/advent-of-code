use std::fs;
use std::collections::HashMap;

struct Node {
    name: String,
    is_small: bool,
}

pub fn run() {
    let input = fs::read_to_string("inputs/input_12.txt")
        .expect("Failed to read file");

    // split input into vector of strings
    let input: Vec<&str> = input
        .split("\n")
        .collect();

    let connections: HashMap<String, Vec<Node>> = get_connections(&input);
    println!("Part 1: {}", part_one(&connections, "start".to_string(), vec!())); // 5252
    println!("Part 2: {}", part_two(&connections, "start".to_string(), vec!(), false)); // 147784
}

fn part_one(connections: &HashMap<String, Vec<Node>>, current_node: String, mut path: Vec<String>) -> u32 {
    let mut count: u32 = 0;

    path.push(current_node.clone());
    let destination_nodes: &Vec<Node> = connections
        .get(&current_node)
        .expect("Key error");

    for node in destination_nodes {
        if node.name == "end" {
            count += 1;
        }

        else if !node.is_small || !path.contains(&node.name) {
            count += part_one(&connections, node.name.clone(), path.clone());
        }
    }

    count
}

fn part_two(connections: &HashMap<String, Vec<Node>>, current_node: String, mut path: Vec<String>, small_repeated: bool) -> u32 {
    let mut count: u32 = 0;
    
    path.push(current_node.clone());

    let destination_nodes: &Vec<Node> = connections
        .get(&current_node)
        .expect("Key error");

    for node in destination_nodes {
        if node.name == "end" {
            count += 1;
        }

        else if !node.is_small || !(small_repeated && path.contains(&node.name)) {
            if small_repeated || (node.is_small && path.contains(&node.name)) {
                count += part_two(&connections, node.name.clone(), path.clone(), true);
            }

            else {
                count += part_two(&connections, node.name.clone(), path.clone(), false);
            }
        }
    }

    count
}

fn get_connections(node_connections: &Vec<&str>) -> HashMap<String, Vec<Node>> {
    let mut connections: HashMap<String, Vec<Node>> = HashMap::new();
    for connection in node_connections {
        let line_split: Vec<&str> = connection
            .split("-")
            .collect();

        let start_node: &str = line_split[0];
        let end_node: &str = line_split[1];

        let start_is_lower: bool = start_node == start_node.to_ascii_lowercase();
        let end_is_lower: bool = end_node == end_node.to_ascii_lowercase();

        if connections.contains_key(&start_node.to_string()) && start_node != "end" && end_node != "start" {
            connections.get_mut(&start_node.to_string())
                .expect("Key error")
                .push(Node {
                name:end_node.to_string(),
                is_small:end_is_lower,
            });
        } else if start_node != "end" && end_node != "start" {
            connections.insert(start_node.to_string(), vec!(Node {
                name:end_node.to_string(),
                is_small:end_is_lower,
            }));
        }

        if connections.contains_key(&end_node.to_string()) && start_node != "start" && end_node != "end" {
            connections.get_mut(&end_node.to_string())
                .expect("Key error")
                .push(Node {
                name:start_node.to_string(),
                is_small:start_is_lower,
            });
        } else if start_node != "start" && end_node != "end" {
            connections.insert(end_node.to_string(), vec!(Node {
                name:start_node.to_string(),
                is_small:start_is_lower,
            }));
        }
    }

    connections
}
