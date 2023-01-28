#[derive(Debug)]
struct Blueprint {
    ore_ore: u32,
    clay_ore: u32,
    obsidian_ore: u32,
    obsidian_clay: u32,
    geode_ore: u32,
    geode_obsidian: u32,
}

impl Blueprint {
    fn can_buy(&self, materials: &Materials, robot: u32) -> bool {
        if robot == 0 && materials.ore >= self.ore_ore ||
            robot == 1 && materials.ore >= self.clay_ore ||
            robot == 2 && materials.ore >= self.obsidian_ore && materials.clay >= self.obsidian_clay ||
            robot == 3 && materials.ore >= self.geode_ore && materials.obsidian >= self.geode_obsidian {
            return true;
        }
        false
    }

    fn buy(&self, materials: &Materials, robots: &Robots, robot: u32) -> Option<(Materials, Robots)> {
        if !self.can_buy(materials, robot) {
            return None;
        }

        match robot {
            // ore robot
            0 => Some((Materials {
                ore: materials.ore - self.ore_ore,
                clay: materials.clay,
                obsidian: materials.obsidian,
                geode: materials.geode
            }, Robots {
                ore: robots.ore + 1,
                clay: robots.clay,
                obsidian: robots.obsidian,
                geode: robots.geode,
            })),

            // clay robot
            1 => Some((Materials {
                ore: materials.ore - self.clay_ore,
                clay: materials.clay,
                obsidian: materials.obsidian,
                geode: materials.geode
            }, Robots {
                ore: robots.ore,
                clay: robots.clay + 1,
                obsidian: robots.obsidian,
                geode: robots.geode,
            })),

            // obsidian robot
            2 => Some((Materials {
                ore: materials.ore - self.obsidian_ore,
                clay: materials.clay - self.obsidian_clay,
                obsidian: materials.obsidian,
                geode: materials.geode
            }, Robots {
                ore: robots.ore,
                clay: robots.clay,
                obsidian: robots.obsidian + 1,
                geode: robots.geode,
            })),

            // geode robot
            3 => Some((Materials {
                ore: materials.ore - self.geode_ore,
                clay: materials.clay,
                obsidian: materials.obsidian - self.geode_obsidian,
                geode: materials.geode
            }, Robots {
                ore: robots.ore,
                clay: robots.clay,
                obsidian: robots.obsidian,
                geode: robots.geode + 1,
            })),

            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct Robots {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Robots {
    fn generate(&self, materials: &mut Materials) {
        materials.ore += self.ore;
        materials.clay += self.clay;
        materials.obsidian += self.obsidian;
        materials.geode += self.geode;
    }
}

#[derive(Clone, Debug)]
struct Materials {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_19.txt");

    let blueprints: Vec<Blueprint> = input_str
        .lines()
        .map(|line| {
            let mut line: Vec<char> = line.chars().skip_while(|&c| c != ':').skip_while(|c| !c.is_ascii_digit()).collect();
            let ore_ore: u32 = line.iter().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();

            line = line.iter().skip_while(|c| c.is_ascii_digit()).skip_while(|c| !c.is_ascii_digit()).copied().collect();
            let clay_ore: u32 = line.iter().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();

            line = line.iter().skip_while(|c| c.is_ascii_digit()).skip_while(|c| !c.is_ascii_digit()).copied().collect();
            let obsidian_ore: u32 = line.iter().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();

            line = line.iter().skip_while(|c| c.is_ascii_digit()).skip_while(|c| !c.is_ascii_digit()).copied().collect();
            let obsidian_clay: u32 = line.iter().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();

            line = line.iter().skip_while(|c| c.is_ascii_digit()).skip_while(|c| !c.is_ascii_digit()).copied().collect();
            let geode_ore: u32 = line.iter().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();

            line = line.iter().skip_while(|c| c.is_ascii_digit()).skip_while(|c| !c.is_ascii_digit()).copied().collect();
            let geode_obsidian: u32 = line.iter().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();

            Blueprint {
                ore_ore,
                clay_ore,
                obsidian_ore,
                obsidian_clay,
                geode_ore,
                geode_obsidian,
            }
        }).collect();

    part_one(&blueprints);
    part_two(&blueprints);
}

fn part_one(blueprints: &[Blueprint]) {
    let mut total_quality_level = 0;
    let mut id = 1;

    for blueprint in blueprints {
        total_quality_level += id * get_most_geodes(blueprint, 24);
        id += 1;
    }

    println!("Part one: {total_quality_level}");
}

fn part_two(blueprints: &[Blueprint]) {
    let mut multiple = 1;

    for blueprint in &blueprints[..3] {
        let geodes = get_most_geodes(blueprint, 32);
        multiple *= geodes;
    }

    println!("Part two: {multiple}");
}

fn get_most_geodes(blueprint: &Blueprint, end_time: u32) -> u32 {
    let mut most_geodes = 0;
    let mut strategies: Vec<(Robots, Materials, u32)> = vec![(
        Robots { ore: 1, clay: 0, obsidian: 0, geode: 0 },
        Materials { ore: 0, clay: 0, obsidian: 0, geode: 0 },
        0,
    )];
    // robots_owned, materials, time

    while !strategies.is_empty() {
        let (robots, materials, time) = strategies.pop().unwrap();

        let geode_at_end = materials.geode + (robots.geode * (end_time - time));
        if geode_at_end > most_geodes {
            most_geodes = materials.geode;
        }
        if time == end_time {
            continue;
        }

        if geode_at_end + (((end_time - time + 1) * (end_time - time)) / 2) < most_geodes {
            continue;
        }

        for goal in 0..=3 {
            if goal == 0 && [blueprint.ore_ore, blueprint.clay_ore, blueprint.obsidian_ore, blueprint.geode_ore].iter().all(|&req| req <= robots.ore) ||
                goal == 1 && robots.clay >= blueprint.obsidian_clay ||
                goal == 2 && robots.obsidian >= blueprint.geode_obsidian ||
                goal == 2 && robots.clay == 0 ||
                goal == 3 && robots.obsidian == 0 {
                continue;
            }

            let mut time = time;
            let mut robots = robots.clone();
            let mut materials = materials.clone();
            let mut done = false;
            while time < end_time && !done {
                if let Some((m, r)) = blueprint.buy(&materials, &robots, goal) {
                    materials = Materials {
                        ore: robots.ore + m.ore,
                        clay: robots.clay + m.clay,
                        obsidian: robots.obsidian + m.obsidian,
                        geode: robots.geode + m.geode,
                    };
                    robots = r;
                    done = true;
                }
                else {
                    robots.generate(&mut materials);
                }
                time += 1;
            }

            strategies.push((robots, materials, time));
        }
    }

    most_geodes
}
