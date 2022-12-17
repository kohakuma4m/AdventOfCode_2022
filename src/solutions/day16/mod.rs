use regex::Regex;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn solution1(data: String) -> u32 {
    let network = read_network_data(data);
    network.print();

    println!("=========================");
    println!("Solution1: ");
    println!("=========================");

    0
}

pub fn solution2(data: String) {
    println!("{}", data);

    println!("=========================");
    println!("Solution2: ");
    println!("=========================");
}

/////////////////////////////////////////////////

#[derive(Debug)]
enum ValveState {
    Open,
    Close
}

#[derive(Debug)]
struct Tunnel {
    length: usize,
    valve: Rc<RefCell<Valve>>
}

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: usize,
    tunnels: Vec<Tunnel>,
    state: ValveState,
    duration: usize
}

// TODO: make Graph library with Node and Edges ?
#[derive(Debug)]
struct ValveNetwork {
    valves: HashMap<String, Rc<RefCell<Valve>>>
}

impl ValveNetwork {
    fn print(&self) {
        for (_, v) in self.valves.iter() {
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
            valves.insert(id.clone(), Rc::new(RefCell::new(Valve { id: id.clone(), flow_rate: 0, tunnels: vec![], state: ValveState::Close, duration: 1 })));
        }

        for other_id in valve_ids.iter() {
            // Creating other valves if they do not already exists
            if valves.contains_key(other_id) == false {
                valves.insert(
                    other_id.clone(),
                    Rc::new(RefCell::new(Valve { id: other_id.clone(), flow_rate: 0, tunnels: vec![], state: ValveState::Close, duration: 1 }))
                );
            }

            // Get valves
            let valve = valves.get(&id).unwrap();
            let other_valve = valves.get(other_id).unwrap();

            // Adding flow rate to current valve
            valve.borrow_mut().flow_rate = flow_rate;

            // Creating tunnel from valve to other valve
            valve.borrow_mut().tunnels.push(Tunnel { valve: Rc::clone(other_valve), length: 1 });

            // Creating tunnel from other_valve to valve
            other_valve.borrow_mut().tunnels.push(Tunnel { valve: Rc::clone(valve), length: 1 });
        }
    });

    ValveNetwork { valves }
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
        assert_eq!((), solution2(data));
    }
}
