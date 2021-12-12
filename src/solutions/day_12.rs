use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    name: String,
    is_small: bool,
}

pub fn run() {
    let input = fs::read_to_string("inputs/input_12.txt")
        .expect("Failed to read file");

    // split input into list of strings
    let input: Vec<&str> = input
        .split("\n")
        .collect();

    part_one(&input);
}

fn part_one(node_connections: &Vec<&str>) {
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
        } else if end_node != "start" && start_node != "end" {
            connections.insert(start_node.to_string(), vec!(Node {
                name:end_node.to_string(),
                is_small:end_is_lower,
            }));
        }

        if connections.contains_key(&end_node.to_string()) && end_node != "end" && start_node != "start" {
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

    let count = path_count("start".to_string(), &connections, vec!("start".to_string()));
    println!("Part 1: {}", count);
}

fn path_count(current_node: String, connections: &HashMap<String, Vec<Node>>, mut path: Vec<String>) -> u32 {
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
            count += path_count(node.name.clone(), &connections, path.clone());
        }
    }

    count
}
