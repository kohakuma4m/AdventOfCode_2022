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

pub fn solution2(data: String) -> usize {
    let mut blueprints = read_blueprints(data).into_iter().filter(|b| b.id <= 3).collect::<Vec<Blueprint>>();

    for blueprint in blueprints.iter_mut() {
        blueprint.print();
        analyze_blueprint(blueprint, 32);
    }

    let result = blueprints.iter().map(|b| b.quality_level / b.id as usize).product();

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
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

impl fmt::Display for FactoryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "robots: {:?}, resources: {:?}, time_left: {}, history: {:?}", self.robots, self.resources, self.time_remaining, self.robot_creation_history)
    }
}

impl FactoryState {
    fn new(time_remaining: usize) -> FactoryState {
        FactoryState { robots: vec![1, 0, 0, 0], resources: vec![0, 0, 0, 0], time_remaining, robot_creation_history: vec![] }
    }

    fn print(&self) {
        println!("State --> robots: {:?}, resources: {:?}", self.robots, self.resources);
        #[cfg(test)]
        for (time, material) in self.robot_creation_history.iter() {
            println!("  > time remaining {:2} --> {:?} robot created", time, material);
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

    fn geode_production_score(&self) -> (usize, usize, usize, usize) {
        (
            // Resource production in order of importance
            self._max_possible_resources(Material::Geode),
            self._max_possible_resources(Material::Obsidian),
            self._max_possible_resources(Material::Clay),
            self._max_possible_resources(Material::Ore)
        )
    }

    fn _max_possible_resources(&self, kind: Material) -> usize {
        // The maximum number of resource possible for current state at the end if a new resource robot could be created each following turn
        let mut max_possible_value = self.resources[kind as usize] + self.robots[kind as usize] * self.time_remaining;
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

// Depth first search solution (completes in less than one minute)
fn analyze_blueprint(blueprint: &mut Blueprint, duration: usize) {
    println!("Analyzing blueprint #{}...", blueprint.id);

    // Building max resource quantity need to build any robot
    let resources_map = build_resource_map(blueprint);

    let mut best_score = (0, 0, 0, 0);
    let mut factory_states = vec![FactoryState::new(duration)];
    let mut processed_states = vec![];
    while factory_states.len() > 0 {
        // Current state
        let mut current_state = factory_states.pop().unwrap();

        // Choose next robot to build (There will always be at least the Geode robot, even after filtering)
        let mut robots_to_build: Vec<Material> = current_state
            .get_buildable_robots(blueprint)
            .iter()
            // Avoid building robots for resource we already produce enough to build any other robot each turn
            .filter(|kind| {
                match resources_map.get(*kind) {
                    Some(max_value) => current_state.robots[**kind as usize] < *max_value,
                    None => true // No robot need geode to build
                }
            })
            .map(|kind| *kind)
            .collect();

        // Avoid building any other robot if we can build a geode robot right now
        if robots_to_build.contains(&Material::Geode) && current_state.can_build_robot(blueprint, Material::Geode) == true {
            robots_to_build = vec![Material::Geode]
        }

        // Future (branches) states: one for each robot to build (the last one will always be for "best" buildable robot that we continue exploring next...)
        let mut nb_new_states = 0;
        for kind in robots_to_build.into_iter() {
            let mut future_state = current_state.clone();

            // Collecting resources...
            while future_state.time_remaining > 0 && future_state.can_build_robot(blueprint, kind) == false {
                future_state.collect_resources(); // ... until we can build chosen robot
            }

            if future_state.time_remaining > 0 {
                // Collecting resources for this turn
                future_state.collect_resources();

                // Building robot
                future_state.build_robot(blueprint, kind);

                factory_states.push(future_state);
                nb_new_states += 1;
            }
        }

        // Terminal (leaf) state: not enough time left to build any new robots...
        if nb_new_states == 0 {
            // Collecting resources until the end
            while current_state.time_remaining > 0 {
                current_state.collect_resources();
            }

            // Updating best current production score
            let max_score = current_state.geode_production_score();
            if max_score > best_score {
                best_score = max_score;
            }

            processed_states.push(current_state);
        }

        // Pruning all states which cannot produce more than the current best score value the end of duration...
        factory_states = factory_states.drain(..).filter(|state| state.geode_production_score() >= best_score).collect();
    }

    // Qualifying blueprint quality
    println!("Analyzing {} states...", processed_states.len());
    if let Some(optimal_state) = processed_states.iter().max_by_key(|s| (s.resources[Material::Geode as usize], s.resources[Material::Obsidian as usize])) {
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
        assert_eq!(56 * 62, solution2(data));
    }
}
