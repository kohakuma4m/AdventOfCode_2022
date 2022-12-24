use std::{collections::HashMap, fmt};

use regex::Regex;

pub fn solution1(data: String) -> usize {
    let mut blueprints = read_blueprints(data);

    for blueprint in blueprints.iter_mut() {
        blueprint.print();
        analyze_blueprint(blueprint, 24);
    }

    let result = blueprints.iter().map(|b| b.quality_level).sum();

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) {
    let blueprints = read_blueprints(data);

    println!("=========================");
    println!("Solution2: ");
    println!("=========================");
}

/////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Material {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3
}

#[derive(Debug, Copy, Clone)]
struct Resource {
    kind: Material,
    quantity: usize
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} (qty: {})", self.kind, self.quantity)
    }
}

#[derive(Debug, Clone)]
struct Robot {
    kind: Material,
    cost: Vec<Resource>
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} robot --> costs: [{}]", self.kind, self.cost.iter().map(|r| format!("{r}")).collect::<Vec<String>>().join(", "))
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u8,
    robots: Vec<Robot>,
    quality_level: usize
}

impl Blueprint {
    fn new(id: u8, robots: Vec<Robot>) -> Blueprint {
        Blueprint { id, robots, quality_level: 0 }
    }

    fn print(&self) {
        println!("");
        println!("Blueprint #{}", self.id);
        println!("---------");
        for robot in self.robots.iter() {
            println!("{}", robot);
        }
        println!("");
    }
}

fn read_blueprints(data: String) -> Vec<Blueprint> {
    let blueprint_regex = Regex::new(
        r"(?x)
        ^
        (?-x)Blueprint (\d+): (?x)
        (?-x)Each ore robot costs (\d+) ore. (?x)
        (?-x)Each clay robot costs (\d+) ore. (?x)
        (?-x)Each obsidian robot costs (\d+) ore and (\d+) clay. (?x)
        (?-x)Each geode robot costs (\d+) ore and (\d+) obsidian.(?x)
        $"
    )
    .unwrap();

    data.lines()
        .map(|line| {
            let captures = blueprint_regex.captures(line).unwrap();
            let id = captures[1].parse::<u8>().unwrap();
            let robots = vec![
                // In order of material
                Robot { kind: Material::Ore, cost: vec![Resource { kind: Material::Ore, quantity: captures[2].parse::<usize>().unwrap() }] },
                Robot { kind: Material::Clay, cost: vec![Resource { kind: Material::Ore, quantity: captures[3].parse::<usize>().unwrap() }] },
                Robot {
                    kind: Material::Obsidian,
                    cost: vec![
                        Resource { kind: Material::Ore, quantity: captures[4].parse::<usize>().unwrap() },
                        Resource { kind: Material::Clay, quantity: captures[5].parse::<usize>().unwrap() },
                    ]
                },
                Robot {
                    kind: Material::Geode,
                    cost: vec![
                        Resource { kind: Material::Ore, quantity: captures[6].parse::<usize>().unwrap() },
                        Resource { kind: Material::Obsidian, quantity: captures[7].parse::<usize>().unwrap() },
                    ]
                },
            ];

            Blueprint::new(id, robots)
        })
        .collect()
}

#[derive(Debug, Clone)]
struct FactoryState {
    robots: Vec<usize>,
    resources: Vec<usize>,
    time_remaining: usize,
    robot_creation_history: Vec<(usize, Material)>
}

impl FactoryState {
    fn new(time_remaining: usize) -> FactoryState {
        FactoryState { robots: vec![1, 0, 0, 0], resources: vec![0, 0, 0, 0], time_remaining, robot_creation_history: vec![] }
    }

    fn print(&self) {
        println!("State --> robots: {:?}, resources: {:?}", self.robots, self.resources);
        #[cfg(test)]
        for (time, material) in self.robot_creation_history.iter() {
            println!("  > time {:2} --> {:?} robot created", time, material);
        }
        println!("------");
    }

    fn get_buildable_robots(&self, blueprint: &Blueprint) -> Vec<Material> {
        let mut materials = vec![];

        for robot in blueprint.robots.iter() {
            if robot.cost.iter().all(|r| self.robots[r.kind as usize] > 0) {
                materials.push(robot.kind); // We produce all resources to build robot eventually...
            }
        }

        materials
    }

    fn can_build_robot(&self, blueprint: &Blueprint, kind: Material) -> bool {
        let robot = &blueprint.robots[kind as usize];
        robot.cost.iter().all(|r| self.resources[r.kind as usize] >= r.quantity)
    }

    fn collect_resources(&mut self) {
        self.time_remaining -= 1;
        for (idx, nb_robots) in self.robots.iter().enumerate() {
            self.resources[idx] += nb_robots;
        }
    }

    fn build_robot(&mut self, blueprint: &Blueprint, kind: Material) {
        let robot = &blueprint.robots[kind as usize];

        self.robots[kind as usize] += 1; // New robot
        for resource in robot.cost.iter() {
            self.resources[resource.kind as usize] -= resource.quantity; // Consuming resources to build new robot
        }

        self.robot_creation_history.push((self.time_remaining, kind));
    }

    fn max_possible_geodes(&self) -> usize {
        // The maximum number of geode possible for current state at the end if a new geode robot could be created each following turn
        let mut max_possible_value = self.resources[Material::Geode as usize] + self.robots[Material::Geode as usize] * self.time_remaining;
        for n in 1..self.time_remaining {
            max_possible_value += self.time_remaining - n // i.e: (time_left - 1)!
        }

        max_possible_value
    }
}

fn build_resource_map(blueprint: &Blueprint) -> HashMap<Material, usize> {
    let mut resources_map = HashMap::new();

    blueprint.robots.iter().for_each(|r| {
        r.cost.iter().for_each(|c| {
            if let Some(value) = resources_map.get(&c.kind) {
                if c.quantity > *value {
                    resources_map.insert(c.kind, c.quantity);
                }
            }
            else {
                resources_map.insert(c.kind, c.quantity);
            }
        });
    });

    resources_map
}

// previous guess: 953 (too low) --> 942 ???

// Depth first search solution
fn analyze_blueprint(blueprint: &mut Blueprint, duration: usize) {
    println!("Analyzing blueprint #{}...", blueprint.id);

    // Building max resource quantity need to build any robot
    let resources_map = build_resource_map(blueprint);

    let mut time = 0;
    let mut factory_states = vec![FactoryState::new(duration)];
    while factory_states.len() > 0 {
        // let max_geodes = factory_states.iter().map(|state| state.max_possible_geodes(duration - time)).max().unwrap();
        // let mut current_states: Vec<FactoryState> = factory_states
        //     .drain(..)
        //     // Pruning all state which cannot produce more than the current best possible value at the end of duration...
        //     .filter(|state| state.max_possible_geodes(duration - time) >= max_geodes)
        //     .collect();

        // Current state
        let mut current_state = factory_states.pop().unwrap();

        // Choose next robot to build
        let robots_to_build = current_state.get_buildable_robots(blueprint);

        // Choose best robot to build, for now, we will analyze other choices later when backtracking...
        let best_robot = robots_to_build.last().unwrap();

        for state in current_states.iter_mut() {
            // Checking which useful robot(s) we can build with current resources (before collecting)
            let robots_to_build: Vec<Material> = state
                .get_robots_to_build(blueprint)
                .iter()
                // Avoid building robots for resource we already produce enough each turn to build any other robot each turn
                .filter(|kind| {
                    match resources_map.get(*kind) {
                        Some(max_value) => state.robots[**kind as usize] < *max_value,
                        None => true // No robot need geode to build
                    }
                })
                .map(|kind| *kind)
                .collect();

            // Collecting resources
            state.collect_resources();

            if robots_to_build.len() == 0 || time == duration {
                // Must wait for more resources (or not enough time) to build another robot...
                factory_states.push(state.clone());
                continue;
            }

            if robots_to_build.contains(&Material::Geode) {
                // We should always prioritize building this robot if we can since it's the one we need to maximize output...
                state.build_robot(blueprint, Material::Geode, time);
                factory_states.push(state.clone());
                continue;
            }

            // Otherwise, we should either build robot or wait for more resources, unless we still don't have a clay robot ???
            if state.robots[Material::Clay as usize] > 0 {
                factory_states.push(state.clone());
            }

            for material in robots_to_build.into_iter() {
                let mut new_state = state.clone();
                new_state.build_robot(blueprint, material, time);
                factory_states.push(new_state);
            }
        }
    }

    // Qualifying blueprint quality
    println!("Analyzing {} states...", factory_states.len());
    if let Some(optimal_state) = factory_states.iter().max_by_key(|s| (s.resources[Material::Geode as usize], s.resources[Material::Obsidian as usize])) {
        optimal_state.print();
        blueprint.quality_level = blueprint.id as usize * optimal_state.resources[Material::Geode as usize];
    }

    println!("Quality level --> {}", blueprint.quality_level);
}

/////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_file() -> String {
        let current_file = std::file!();
        let test_file = current_file.replace("mod.rs", "test.txt");
        return crate::read_file(&test_file).unwrap();
    }

    #[test]
    fn test_solution1() {
        let data = read_test_file();
        assert_eq!(33, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!((), solution2(data));
    }
}
