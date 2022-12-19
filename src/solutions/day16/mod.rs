use itertools::Itertools;
use regex::Regex;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc
};

// TODO: make generic Graph library with Node and Edges ?

pub fn solution1(data: String) -> usize {
    let network = read_network_data(data);
    network.print();

    let paths = map_all_shortest_paths(&network);

    #[cfg(test)]
    for ((v1, v2), p) in paths.iter().sorted_by_key(|(key, _)| *key) {
        println!("{} --> {} (time needed: {}) --> {:?}", v1, v2, p.weight, p.locations);
    }

    let path = find_optimal_path(&network, &paths, "AA", 30);
    let result = path.released_pressure;

    println!("=========================");
    println!("Path: {:?}", path.locations);
    println!(
        "Opened valves history: {:?}",
        path.opened_valves_history.iter().sorted_by_key(|(time, _)| -(**time as isize)).collect::<Vec<(&usize, &String)>>()
    );
    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> usize {
    println!("{}", data);

    println!("=========================");
    println!("Solution2: ");
    println!("=========================");

    0
}

/////////////////////////////////////////////////

#[derive(Debug)]
struct Tunnel {
    time_to_traverse: usize,
    valve: Rc<RefCell<Valve>>
}

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: usize,
    tunnels: Vec<Tunnel>,
    time_to_open: usize
}

#[derive(Debug)]
struct ValveNetwork {
    valves: HashMap<String, Rc<RefCell<Valve>>>
}

impl ValveNetwork {
    fn print(&self) {
        for (_, v) in self.valves.iter().sorted_by_key(|(key, _)| *key) {
            println!(
                "{} ({:<2}) --> {:?}",
                v.as_ref().borrow().id,
                v.as_ref().borrow().flow_rate,
                v.as_ref().borrow().tunnels.iter().map(|v2| format!("{}", v2.valve.as_ref().borrow().id)).collect::<Vec<String>>()
            );
        }
    }
}

fn read_network_data(data: String) -> ValveNetwork {
    let valve_regex = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (([A-Z]{2},? ?)*)$").unwrap();

    let mut valves = HashMap::new();
    data.lines().for_each(|line| {
        let captures = valve_regex.captures(line).unwrap();
        let id = String::from(&captures[1]);
        let flow_rate = captures[2].parse::<usize>().unwrap();
        let valve_ids: Vec<String> = captures[3].split(", ").map(|s| String::from(s)).collect();

        // Creating valve is it does not already exists
        if valves.contains_key(&id) == false {
            valves.insert(id.clone(), Rc::new(RefCell::new(Valve { id: id.clone(), flow_rate: 0, tunnels: vec![], time_to_open: 1 })));
        }

        for other_id in valve_ids.iter() {
            // Creating other valves if they do not already exists
            if valves.contains_key(other_id) == false {
                valves.insert(other_id.clone(), Rc::new(RefCell::new(Valve { id: other_id.clone(), flow_rate: 0, tunnels: vec![], time_to_open: 1 })));
            }

            // Get valves
            let valve = valves.get(&id).unwrap();
            let other_valve = valves.get(other_id).unwrap();

            // Adding flow rate to current valve
            valve.borrow_mut().flow_rate = flow_rate;

            // Creating tunnel from valve to other valve
            valve.borrow_mut().tunnels.push(Tunnel { valve: Rc::clone(other_valve), time_to_traverse: 1 });
        }
    });

    ValveNetwork { valves }
}

#[derive(Debug, Clone)]
struct PartialPath {
    locations: Vec<String>,
    weight: usize
}

fn map_all_shortest_paths(network: &ValveNetwork) -> HashMap<(String, String), PartialPath> {
    let mut paths: HashMap<(String, String), PartialPath> = HashMap::new();

    for valve_id in network.valves.keys() {
        for other_valve_id in network.valves.keys() {
            let other_valve = network.valves.get(other_valve_id).unwrap().as_ref().borrow();
            if other_valve_id == valve_id || other_valve.flow_rate == 0 {
                continue; // No point in finding path to non destination valves
            }

            let path = find_shortest_path(network, valve_id, other_valve_id).unwrap();
            paths.insert((valve_id.clone(), other_valve_id.clone()), path);
        }
    }

    paths
}

fn find_shortest_path(network: &ValveNetwork, start: &String, goal: &String) -> Option<PartialPath> {
    let mut visited_locations: HashSet<String> = HashSet::new();
    let mut paths_to_explored: Vec<PartialPath> = vec![PartialPath { locations: vec![start.clone()], weight: 0 }];

    while paths_to_explored.len() > 0 {
        let current_paths: Vec<PartialPath> = paths_to_explored.drain(..).collect();

        for path in current_paths {
            let current_location = path.locations.last().unwrap();
            let current_valve = network.valves.get(current_location).unwrap().as_ref().borrow();

            for tunnel in current_valve.tunnels.iter() {
                let valve = tunnel.valve.as_ref().borrow();
                if visited_locations.contains(&valve.id) {
                    continue; // Ignoring already visited locations
                }

                let mut next_path = PartialPath { locations: path.locations.clone(), weight: path.weight + tunnel.time_to_traverse };
                next_path.locations.push(valve.id.clone());

                if valve.id == *goal {
                    return Some(next_path);
                }

                visited_locations.insert(valve.id.clone());
                paths_to_explored.push(next_path);
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct GlobalPath {
    locations: Vec<String>,
    opened_valves_history: HashMap<usize, String>,
    remaining_valves: HashSet<String>,
    remaining_time: usize,
    released_pressure: usize
}

fn find_optimal_path(network: &ValveNetwork, paths: &HashMap<(String, String), PartialPath>, start: &str, max_duration: usize) -> GlobalPath {
    let mut found_paths: Vec<GlobalPath> = vec![];
    let mut path_to_explored: Vec<GlobalPath> = vec![GlobalPath {
        locations: vec![String::from(start)],
        opened_valves_history: HashMap::new(),
        remaining_valves: HashSet::from_iter(network.valves.keys().map(|k| k.clone()).filter(|id| {
            // Excluding all destination valves with zero flow rate, as we won't waste time opening them
            network.valves.get(id).unwrap().as_ref().borrow().flow_rate > 0
        })),
        remaining_time: max_duration,
        released_pressure: 0
    }];

    println!("-------------------------");
    while path_to_explored.len() > 0 {
        println!("Number of paths to explore: {}", path_to_explored.len());

        let mut current_paths: Vec<GlobalPath> = path_to_explored.drain(..).collect();
        for path in current_paths.iter_mut() {
            // Current valve
            let current_valve = network.valves.get(path.locations.last().unwrap()).unwrap().as_ref().borrow();
            if current_valve.flow_rate > 0 {
                // Opening current valve
                path.remaining_time -= current_valve.time_to_open;
                path.opened_valves_history.insert(path.remaining_time, current_valve.id.clone()); // Time left to release pressure (temp for debug)
                path.released_pressure += path.remaining_time * current_valve.flow_rate; // Total pressure released by this valve at the end
                path.remaining_valves.remove(&current_valve.id);

                if path.remaining_time == 0 || path.remaining_valves.len() == 0 {
                    // Time's up or every valves are already opened
                    found_paths.push(path.clone());
                    continue;
                }
            }

            // Next valves
            let mut nb_next_paths = 0;
            for valve_id in path.remaining_valves.iter() {
                if *valve_id == current_valve.id {
                    continue; // Ignoring current valve
                }

                let partial_path = paths.get(&(current_valve.id.clone(), valve_id.clone())).unwrap();
                if path.remaining_time < partial_path.weight + network.valves.get(valve_id).unwrap().as_ref().borrow().time_to_open {
                    // Not enough time left to go to next valve and open it...
                    continue;
                }

                let mut new_path = path.clone();
                new_path.remaining_time -= partial_path.weight;
                for other_id in &partial_path.locations[1..] {
                    new_path.locations.push(other_id.clone());
                }
                path_to_explored.push(new_path);
                nb_next_paths += 1;
            }

            if nb_next_paths == 0 {
                // Not enough time left to go to any other valve and open it...
                found_paths.push(path.clone());
            }
        }
    }
    println!("-------------------------");
    println!("Number of paths found: {}", found_paths.len());

    // Return optimal path (will always exists)
    found_paths.into_iter().max_by_key(|p| p.released_pressure).unwrap()
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
        assert_eq!(1651, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(1707, solution2(data));
    }
}
