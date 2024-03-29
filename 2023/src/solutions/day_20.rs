use std::collections::{HashMap, VecDeque};

use crate::{safe_unpack, common::maths::lcm};

type Parsed = HashMap<String, Module>;

#[derive(Clone)]
pub struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
} 

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut parsed = HashMap::new();
    for line in input_raw.lines() {
        safe_unpack!(line.split(" -> "), name, destinations);

        let destinations = destinations
            .split(", ")
            .map(|dest| dest.to_string())
            .collect();

        if name == "broadcaster" {
            parsed.insert(name.to_string(), Module {
                module_type: ModuleType::Broadcast,
                destinations,
            });
        } else if name.starts_with('&') {
            parsed.insert(name[1..].to_string(), Module {
                module_type: ModuleType::Conjunction(HashMap::new()),
                destinations,
            });
        } else if name.starts_with('%') {
            parsed.insert(name[1..].to_string(), Module {
                module_type: ModuleType::FlipFlop(false),
                destinations,
            });
        } else {
            panic!();
        }
    }
    for (name, module) in parsed.clone() {
        for dest in module.destinations {
            if let Some(dest_module) = parsed.get_mut(&dest) {
                if let ModuleType::Conjunction(mut inputs) = dest_module.module_type.clone() {
                    inputs.insert(name.clone(), false);
                    dest_module.module_type = ModuleType::Conjunction(inputs);
                }
            }
        }
    }
    Ok(parsed)
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let mut modules = input.to_owned();

    let mut steps = VecDeque::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut buttons_pressed = 0;
    while buttons_pressed < 1000 {
        buttons_pressed += 1;
        low_pulses += 1;
        steps.push_back((String::from("broadcaster"), false));
        while !steps.is_empty() {
            let (name, pulse) = steps.pop_front().unwrap();
            let mut updated_modules = modules.clone();
            if let Some(module) = modules.get(&name) {
                let pulse_to_send = match module.module_type.clone() {
                    ModuleType::Broadcast => pulse,
                    ModuleType::Conjunction(inputs) => inputs.values().any(|&i| !i),
                    ModuleType::FlipFlop(state) => if !pulse {
                        updated_modules.get_mut(&name).unwrap().module_type = ModuleType::FlipFlop(!state);
                        !state
                    } else {
                        continue
                    },
                };

                for dest in &*module.destinations {
                    steps.push_back((dest.to_string(), pulse_to_send));
                    if pulse_to_send {
                        high_pulses += 1;
                    }
                    else {
                        low_pulses += 1;
                    }
                    if let Some(dest_module) = updated_modules.get_mut(dest) {
                        if let ModuleType::Conjunction(mut inputs) = dest_module.module_type.clone() {
                            *inputs.entry(name.clone()).or_insert(false) = pulse_to_send;
                            dest_module.module_type = ModuleType::Conjunction(inputs);
                        }
                    }
                }
            }
            modules = updated_modules;
        }
    }
    Ok(low_pulses * high_pulses)
}

// done with the help of https://youtu.be/lxm6i21O83k (HyperNeutrino on YouTube)
pub fn part_two(input: &Parsed) -> Result<u64, String> {
    let mut modules = input.clone();

    let rx_connection = input
        .iter()
        .find_map(|(name, module)| if module.destinations.contains(&String::from("rx")) {
            Some(name)
        } else {
            None
        })
        .unwrap();
    
    let mut connections_to_rx_connection = 0;
    for (_, module) in input {
        if module.destinations.contains(rx_connection) {
            connections_to_rx_connection += 1;
        }
    }

    let mut cycle_lengths = HashMap::new();

    let start_name = "broadcaster".to_string();
    let start_module = input.get(&start_name).unwrap();

    let mut presses = 0;
    loop {
        presses += 1;
        let mut queue = VecDeque::new();
        for module in start_module.destinations.clone() {
            queue.push_back((start_name.to_string(), module, false));
        }
        while !queue.is_empty() {
            let (origin, target, pulse) = queue.pop_front().unwrap();
            let mut updated_modules = modules.clone();
            if let Some(module) = modules.get(&target) {
                let pulse_to_send = match module.module_type.clone() {
                    ModuleType::Broadcast => pulse,
                    ModuleType::Conjunction(inputs) => inputs.values().any(|&i| !i),
                    ModuleType::FlipFlop(state) => if !pulse {
                        updated_modules.get_mut(&target).unwrap().module_type = ModuleType::FlipFlop(!state);
                        !state
                    } else {
                        continue
                    },
                };

                for dest in &*module.destinations {
                    queue.push_back((target.clone(), dest.to_string(), pulse_to_send));
                    if let Some(dest_module) = updated_modules.get_mut(dest) {
                        if let ModuleType::Conjunction(mut inputs) = dest_module.module_type.clone() {
                            *inputs.entry(target.clone()).or_insert(false) = pulse_to_send;
                            dest_module.module_type = ModuleType::Conjunction(inputs);
                        }
                    }
                }
            }
            modules = updated_modules;

            if &target == rx_connection && pulse {
                if !cycle_lengths.contains_key(&origin) {
                    cycle_lengths.insert(origin, presses);
                }
            }
        }

        if connections_to_rx_connection == cycle_lengths.len() {
            break;
        }
    }

    let mut total_presses = 1;
    for i in cycle_lengths.values() {
        total_presses = lcm(total_presses, *i);
    }

    Ok(total_presses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_20_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(11687500));
    }

    // no example for testing part 2 :(
}
