use std::{collections::HashMap, rc::Rc, cell::RefCell};

use itertools::Itertools;
use regex::Regex;

pub fn solution1(data: String) -> isize {
    let mut monkeys_map = read_monkey_data(data, false);
    let result = find_root_number(&mut monkeys_map);

    for (name, monkey) in monkeys_map.iter().sorted_by_key(|(name, _)| *name) {
        println!("{} --> {:?}", name, monkey.as_ref().borrow().yelled_number);
    }

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> isize {
    let mut monkeys_map = read_monkey_data(data, true);
    let result = find_human_number(&mut monkeys_map, 30000);

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

struct Monkey {
    _name: String,
    yelled_number: Option<isize>,
    operation: Option<Box<dyn Fn(&HashMap<String, Rc<RefCell<Monkey>>>) -> Option<isize>>>
}

fn read_monkey_data(data: String, fix_data: bool) -> HashMap<String, Rc<RefCell<Monkey>>> {
    let mut monkeys_map = HashMap::new();

    let monkey_regex = Regex::new(r"^(\w+): (?:(\d+)|(\w+) (.) (\w+))$").unwrap();
    data.lines().for_each(|line| {
        let captures = monkey_regex.captures(line).unwrap();

        let name = String::from(&captures[1]);
        if let Some(group) = captures.get(2) {
            let monkey = if fix_data && name == "root" {
                Monkey { _name: name.clone(), yelled_number: None, operation: None }
            }
            else {
                Monkey { _name: name.clone(), yelled_number: Some(group.as_str().parse::<isize>().unwrap()), operation: None }
            };
            monkeys_map.insert(name, Rc::new(RefCell::new(monkey)));
            return;
        }

        let op = &captures[4];
        let name1 = String::from(&captures[3]);
        let name2 = String::from(&captures[5]);
        let operation: Box<dyn Fn(&HashMap<String, Rc<RefCell<Monkey>>>) -> Option<isize>>;
        if fix_data && name == "root" {
            operation = Box::new(move |numbers: &HashMap<String, Rc<RefCell<Monkey>>>| -> Option<isize> {
                match (numbers.get(&name1).unwrap().as_ref().borrow().yelled_number, numbers.get(&name2).unwrap().as_ref().borrow().yelled_number) {
                    (Some(a), Some(b)) => Some(if a == b { 1 } else { 0 }),
                    _ => None
                }
            });
        }
        else {
            operation = match op {
                "+" => Box::new(move |numbers: &HashMap<String, Rc<RefCell<Monkey>>>| -> Option<isize> {
                    match (numbers.get(&name1).unwrap().as_ref().borrow().yelled_number, numbers.get(&name2).unwrap().as_ref().borrow().yelled_number) {
                        (Some(a), Some(b)) => Some(a + b),
                        _ => None
                    }
                }),
                "-" => Box::new(move |numbers: &HashMap<String, Rc<RefCell<Monkey>>>| -> Option<isize> {
                    match (numbers.get(&name1).unwrap().as_ref().borrow().yelled_number, numbers.get(&name2).unwrap().as_ref().borrow().yelled_number) {
                        (Some(a), Some(b)) => Some(a - b),
                        _ => None
                    }
                }),
                "*" => Box::new(move |numbers: &HashMap<String, Rc<RefCell<Monkey>>>| -> Option<isize> {
                    match (numbers.get(&name1).unwrap().as_ref().borrow().yelled_number, numbers.get(&name2).unwrap().as_ref().borrow().yelled_number) {
                        (Some(a), Some(b)) => Some(a * b),
                        _ => None
                    }
                }),
                "/" => Box::new(move |numbers: &HashMap<String, Rc<RefCell<Monkey>>>| -> Option<isize> {
                    match (numbers.get(&name1).unwrap().as_ref().borrow().yelled_number, numbers.get(&name2).unwrap().as_ref().borrow().yelled_number) {
                        (Some(a), Some(b)) => Some(a / b),
                        _ => None
                    }
                }),
                _ => panic!("Invalid operation '{op}'")
            };
        }

        let monkey = Monkey { _name: name.clone(), yelled_number: None, operation: Some(operation) };
        monkeys_map.insert(name, Rc::new(RefCell::new(monkey)));
    });

    monkeys_map
}

fn find_root_number(monkeys_map: &mut HashMap<String, Rc<RefCell<Monkey>>>) -> isize {
    // We could build a tree from root monkey, but it's quicker just to unfold values until root is filled
    loop {
        monkeys_map.values()
            .filter(|monkey| monkey.as_ref().borrow().yelled_number == None)
            .for_each(|monkey_ref| {
                let mut monkey = monkey_ref.as_ref().borrow_mut();

                if let Some(operation ) = &monkey.operation {
                    monkey.yelled_number = (operation)(&monkeys_map);
                }
            });

        if let Some(value) = monkeys_map.get("root").unwrap().as_ref().borrow().yelled_number {
            return value;
        }
    }
}

// Trying all human positive number sequentially until right and hoping answer is not too big... nope :(
// TODO: Build binary tree from root down keeping track of both sides until reaching human to compare ???
fn find_human_number(monkeys_map: &mut HashMap<String, Rc<RefCell<Monkey>>>, starting_guess: isize) -> isize {
    let mut human_number: isize = starting_guess;

    loop {
        if human_number % 1000 == 0 {
            println!("Guessing value {human_number}...");
        }

        loop {
            // Set human number
            monkeys_map.get("humn").unwrap().as_ref().borrow_mut().yelled_number = Some(human_number);

            monkeys_map.values()
                .filter(|monkey| monkey.as_ref().borrow().yelled_number == None)
                .for_each(|monkey_ref| {
                    let mut monkey = monkey_ref.as_ref().borrow_mut();

                    if let Some(operation ) = &monkey.operation {
                        monkey.yelled_number = (operation)(&monkeys_map);
                    }
                });

            match monkeys_map.get("root").unwrap().as_ref().borrow().yelled_number {
                Some(1) => return human_number,
                Some(_) => break,
                None => continue
            }
        }

        // Reset and try again
        monkeys_map.values()
            .for_each(|monkey_ref| {
                let mut monkey = monkey_ref.as_ref().borrow_mut();

                if let Some(_) = &monkey.operation {
                    monkey.yelled_number = None;
                }
            });
        human_number += 1;
    }
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
        assert_eq!(152, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(301, solution2(data));
    }
}
