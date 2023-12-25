extern crate rand;

use std::collections::HashSet;

use self::rand::Rng;

use crate::safe_unpack;

type Parsed = (Vec<Node>, Vec<Edge>);
type Node = String;
type Edge = (Node, Node);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut nodes: HashSet<Node> = HashSet::new();
    let mut edges: Vec<Edge> = vec![];
    for line in input_raw.lines() {
        safe_unpack!(line.split(": "), node, destinations);

        nodes.insert(node.to_string());
        for dest in destinations.split_ascii_whitespace() {
            let mut unique = true;
            for edge in &edges {
                nodes.insert(edge.0.to_string());
                nodes.insert(edge.1.to_string());

                if edge == &(node.to_string(), dest.to_string()) || edge == &(dest.to_string(), node.to_string()) {
                    unique = false;
                }
            }

            if unique {
                edges.push((node.to_string(), dest.to_string()));
            }
        }
    }

    Ok((nodes.into_iter().collect(), edges))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (nodes, edges) = input;
    println!("{}, {}", nodes.len(), edges.len());
    
    let mut split = [0, 0];
    let mut cut_size = 0;
    while cut_size != 3 {
        let mut nodes = nodes.clone();
        let mut edges = edges.clone();
        kargers_algorithm(&mut nodes, &mut edges);

        let mut cut = vec![];
        for edge in &edges {
            if edge.0 != edge.1 {
                cut.push(edge);
            }
        }
        split = [nodes[0].len() / 3, nodes[1].len() / 3];
        cut_size = cut.len();
        println!("{split:?}, {cut_size}");
    }

    Ok(split[0] * split[1])
}

pub fn part_two(_: &Parsed) -> Result<&str, String> {
    Ok("MERRY CHRISTMAS!")
}

fn kargers_algorithm(nodes: &mut Vec<Node>, edges: &mut Vec<Edge>) {
    while nodes.len() > 2 {
        let edge = edges[rand::thread_rng().gen_range(0..edges.len())].clone();

        if edge.0 == edge.1 {
            continue;
        }

        contract_edge(&edge, nodes, edges);
    }
}

fn contract_edge(edge: &Edge, nodes: &mut Vec<Node>, edges: &mut Vec<Edge>) {
    nodes.remove(nodes.iter().position(|n| n == &edge.0).unwrap());
    nodes.remove(nodes.iter().position(|n| n == &edge.1).unwrap());

    let new_node = format!("{}{}", edge.0, edge.1);
    nodes.push(new_node.clone());

    for other_edge in edges.iter_mut() {
        if other_edge.0 == edge.0 || other_edge.0 == edge.1 {
            other_edge.0 = new_node.clone();
        }

        if other_edge.1 == edge.0 || other_edge.1 == edge.1 {
            other_edge.1 = new_node.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_25_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(54));
    }
}
