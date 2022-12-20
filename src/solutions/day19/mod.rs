use std::{fmt, collections::HashMap};

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
    history: Vec<(u32, Material)>
}

impl FactoryState {
    fn new() -> FactoryState {
        FactoryState { robots: vec![1, 0, 0, 0], resources: vec![0, 0, 0, 0], history: vec![] }
    }

    fn print(&self) {
        println!("State --> robots: {:?}, resources: {:?}", self.robots, self.resources);
        #[cfg(test)]
        for (time, material) in self.history.iter() {
            println!("  > time {:2} --> {:?} robot created", time, material);
        }
        println!("------");
    }

    fn collect_resources(&mut self) {
        for (idx, nb_robots) in self.robots.iter().enumerate() {
            self.resources[idx] += nb_robots;
        }
    }

    fn get_robots_to_build(&self, blueprint: &Blueprint) -> Vec<Material> {
        let mut materials = vec![];

        for robot in blueprint.robots.iter() {
            if robot.cost.iter().all(|r| self.resources[r.kind as usize] >= r.quantity) {
                materials.push(robot.kind); // Enough resources available to build robot
            }
        }

        materials
    }

    fn build_robot(&mut self, blueprint: &Blueprint, kind: Material, time: u32) {
        let robot = &blueprint.robots[kind as usize];

        self.robots[kind as usize] += 1; // New robot
        for resource in robot.cost.iter() {
            self.resources[resource.kind as usize] -= resource.quantity; // Consuming resources to build new robot
        }

        self.history.push((time, kind));
    }
}

fn build_resource_map(blueprint: &Blueprint) -> HashMap<Material, usize> {
    let mut resources_map = HashMap::new();

    blueprint.robots.iter().for_each(|r| {
        r.cost.iter().for_each (|c| {
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

fn analyze_blueprint(blueprint: &mut Blueprint, duration: u32) {
    println!("Analyzing blueprint #{}...", blueprint.id);

    // Building max resource quantity need to build any robot
    let resources_map = build_resource_map(blueprint);

    let mut time = 0;
    let mut factory_states = vec![FactoryState::new()];
    while time < duration && factory_states.len() > 0 {
        time += 1;

        let max_geodes = factory_states.iter().map(|state| state.resources[Material::Geode as usize]).max().unwrap();
        let mut current_states: Vec<FactoryState> = factory_states.drain(..)
            // Pruning all state which have less than the best current number of geode, since they will never be able to catch up with best current partial solution...
            .filter(|state| state.resources[Material::Geode as usize] == max_geodes )
            .collect();

        #[cfg(test)]
        println!("Minute {:2} --> {} states to analyze", time, current_states.len());

        for state in current_states.iter_mut() {
            // Checking which useful robot(s) we can build with current resources (before collecting)
            let robots_to_build: Vec<Material> = state.get_robots_to_build(blueprint).iter()
                // Filtering to not build robot for resource we already produce enough each turn to build any other robot
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

            if robots_to_build.contains(&Material::Geode) | robots_to_build.contains(&Material::Obsidian) {
                // We should always prioritize building those robots first if we can
                let best_robot_to_build = robots_to_build.last().unwrap(); // Only building geode robot if we have both
                state.build_robot(blueprint, *best_robot_to_build, time);
                factory_states.push(state.clone());
                continue;
            }

            // Otherwise, we should either build robot or wait for more resources, unless we don't yet have a clay robot yet
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
    if let Some(optimal_state) = factory_states.iter().max_by_key(|s| {
        (s.resources[Material::Geode as usize], s.resources[Material::Obsidian as usize])
    }) {
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
