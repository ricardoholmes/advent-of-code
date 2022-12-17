use std::collections::{HashMap, BTreeSet};

#[derive(Clone)]
struct Tunnel {
    flow_rate: i32,
    connections: Vec<String>,
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_16.txt");

    let input: HashMap<String, Tunnel> = input_str
        .lines()
        .map(|line| {
            // hard coding because yes
            let name: String = line.chars().skip(6).take(2).collect();

            let line = &line[line.find('=').unwrap()+1..];
            let flow_rate: i32 = line.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
            let connections: Vec<String> = line[line.find("valve").unwrap()..].split(&[' ', ',']).filter(|c| !c.is_empty()).skip(1).map(|i| i.to_string()).collect();

            (name, Tunnel {
                flow_rate,
                connections,
            })
        }).collect();

    // Floyd-Warshall Algorithm
    // generates distances between all nodes
    let mut dist: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for u in input.keys() {
        let mut connections: HashMap<String, i32> = HashMap::new();
        for v in input.keys() {
            connections.insert(v.clone(), u16::MAX as i32);
        }
        for v in &input.get(u).unwrap().connections {
            connections.insert(v.clone(), 1);
        }
        connections.insert(u.to_string(), 0);
        dist.insert(u.clone(), connections);
    }

    for k in input.keys() {
        for i in input.keys() {
            for j in input.keys() {
                if *dist.get(i).unwrap().get(j).unwrap() > dist.get(i).unwrap().get(k).unwrap() + dist.get(k).unwrap().get(j).unwrap() {
                    let new_weight = dist.get(i).unwrap().get(k).unwrap() + dist.get(k).unwrap().get(j).unwrap();
                    let mut connections = dist.get(i).unwrap().clone();
                    connections.insert(j.to_string(), new_weight);
                    dist.insert(i.to_string(), connections.clone());
                }
            }
        }
    }

    part_one(&input, &dist);
    part_two(&input, &dist);
}

fn part_one(tunnels: &HashMap<String, Tunnel>, dist: &HashMap<String, HashMap<String, i32>>) {
    let mut best_paths: HashMap<BTreeSet<String>, i32> = HashMap::new();
    let mut best_score = i32::MIN;
    let mut paths: Vec<(String, i32, i32, BTreeSet<String>)> = vec![("AA".to_owned(), 0, 0, BTreeSet::new())];
    // paths is Vec<(valve, minutes, total_flow, visited)>

    while paths.len() > 0 {
        let (node, minutes, total_flow, mut visited) = paths.pop().unwrap();
        visited.insert(node.clone());
        let total_flow_rate = visited
            .iter()
            .fold(0, |total, node| total + tunnels.get(node).unwrap().flow_rate);

        let mut new_paths: Vec<(String, i32, i32, BTreeSet<String>)> = vec![];
        for (dest, distance) in dist.get(&node).unwrap() {
            let time = minutes + distance + 1;
            if tunnels.get(dest).unwrap().flow_rate != 0 && time < 30 && !visited.contains(dest) {
                new_paths.push((
                    dest.clone(),
                    time,
                    total_flow + total_flow_rate * (distance + 1),
                    visited.clone()
                ));
            }
        }

        if new_paths.len() == 0 {
            let total_flow = total_flow + total_flow_rate * (30 - minutes);
            if !best_paths.contains_key(&visited) || best_paths.get(&visited).unwrap() < &total_flow {
                best_paths.insert(visited, total_flow);
                if total_flow > best_score {
                    best_score = total_flow;
                }
            }
        }
        else {
            paths.extend(new_paths);
        }
    }

    println!("Part one: {best_score}");
}

fn part_two(tunnels: &HashMap<String, Tunnel>, dist: &HashMap<String, HashMap<String, i32>>) {
    let mut best_score = i32::MIN;
    let mut paths: Vec<(String, String, i32, i32, i32, BTreeSet<String>)> = vec![("AA".to_owned(), "AA".to_owned(), 0, 0, 0, BTreeSet::new())];

    while paths.len() > 0 {
        let (node, elephant_node, minutes, elephant_minutes, total_flow, mut visited) = paths.pop().unwrap();
        let time = minutes.min(elephant_minutes);

        if time == minutes {
            visited.insert(node.clone());
        }
        if time == elephant_minutes {
            visited.insert(elephant_node.clone());
        }

        let total_flow_rate = visited
            .iter()
            .fold(0, |total, point| total + tunnels.get(point).unwrap().flow_rate);

        let mut new_paths: Vec<(String, String, i32, i32, i32, BTreeSet<String>)> = vec![];
        if time == minutes {
            for (dest, distance) in dist.get(&node).unwrap() {
                let stop_time = minutes + distance + 1;
                if tunnels.get(dest).unwrap().flow_rate != 0 && stop_time < 26 && *dest != elephant_node && !visited.contains(dest) {
                    new_paths.push((
                        dest.clone(),
                        elephant_node.clone(),
                        stop_time,
                        elephant_minutes,
                        total_flow + total_flow_rate * (distance + 1).min(elephant_minutes - time),
                        visited.clone()
                    ));
                }
            }
        }

        if time == elephant_minutes {
            for (dest, distance) in dist.get(&elephant_node).unwrap() {
                let stop_time = elephant_minutes + distance + 1;
                if tunnels.get(dest).unwrap().flow_rate != 0 && stop_time < 26 && *dest != node && !visited.contains(dest) {
                    new_paths.push((
                        node.clone(),
                        dest.clone(),
                        minutes,
                        stop_time,
                        total_flow + total_flow_rate * (distance + 1).min(minutes - time),
                        visited.clone()
                    ));
                }
            }
        }

        if new_paths.len() == 0 {
            visited.insert(node.clone());
            visited.insert(elephant_node.clone());

            let total_flow_rate = visited
                .iter()
                .fold(0, |total, point| 
                    total + if *point != node && *point != elephant_node { tunnels.get(point).unwrap().flow_rate } else { 0 }
                );

            let new_total_flow = total_flow
                + total_flow_rate * (26 - minutes.min(elephant_minutes))
                + tunnels.get(&node).unwrap().flow_rate * (26 - minutes)
                + tunnels.get(&elephant_node).unwrap().flow_rate * (26 - elephant_minutes);

            if new_total_flow > best_score {
                best_score = new_total_flow;
                println!("{best_score}");
            }
        }
        else {
            paths.extend(new_paths);
        }
    }

    println!("Part two: {best_score}");
}
