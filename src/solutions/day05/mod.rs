use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn solution1(data: String) -> String {
    let (stacks, moves) = read_stacks_and_moves(data);

    for m in moves {
        for _n in 0..m.nb_crates {
            let crate_name = stacks.get(&m.source_stack_id).unwrap().crates.borrow_mut().pop().unwrap();
            stacks.get(&m.target_stack_id).unwrap().crates.borrow_mut().push(crate_name);
        }
    }

    let mut result = String::new();
    for n in 0..stacks.len() {
        result.push_str(&stacks.get(&((n as i8) + 1)).unwrap().crates.borrow().last().unwrap().to_string());
    }

    print_stacks(&stacks);

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> String {
    let (stacks, moves) = read_stacks_and_moves(data);

    for m in moves {
        let mut crates: Vec<char> = vec![];
        for _n in 0..m.nb_crates {
            let crate_name = stacks.get(&m.source_stack_id).unwrap().crates.borrow_mut().pop().unwrap();
            crates.push(crate_name);
        }
        for c in crates.iter().rev() {
            stacks.get(&m.target_stack_id).unwrap().crates.borrow_mut().push(*c);
        }
    }

    let mut result = String::new();
    for n in 0..stacks.len() {
        result.push_str(&stacks.get(&((n as i8) + 1)).unwrap().crates.borrow().last().unwrap().to_string());
    }

    print_stacks(&stacks);

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug)]
struct Stack {
    crates: RefCell<Vec<char>>
}

#[derive(Debug)]
struct Move {
    nb_crates: i32,
    source_stack_id: i8,
    target_stack_id: i8
}

fn read_stacks_and_moves(data: String) -> (HashMap<i8, Stack>, Vec<Move>) {
    let lines: Vec<&str> = data.lines().collect();
    let empty_line_idx = lines.iter().position(|&l| l == "").unwrap();

    // Reading stacks
    let mut stacks: HashMap<i8, Stack> = HashMap::new();
    for n in lines[empty_line_idx - 1].split_whitespace() {
        let stack_id = n.parse::<i8>().unwrap();
        stacks.insert(stack_id, Stack { crates: RefCell::new(vec![]) });
    }
    for idx in (0..empty_line_idx - 1).rev() {
        let chars: Vec<char> = lines[idx].chars().collect();
        let line_length = chars.len();
        for n in 0..stacks.len() {
            let crate_name_idx = 4 * n + 1;
            if crate_name_idx < line_length && chars[crate_name_idx] != ' ' {
                stacks.get(&((n as i8) + 1)).unwrap().crates.borrow_mut().push(chars[crate_name_idx]);
            }
        }
    }

    // Reading moves
    let mut moves: Vec<Move> = vec![];
    let move_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for idx in empty_line_idx + 1..lines.len() {
        let captures = move_regex.captures(lines[idx]).unwrap();
        moves.push(Move {
            nb_crates: captures[1].parse::<i32>().unwrap(),
            source_stack_id: captures[2].parse::<i8>().unwrap(),
            target_stack_id: captures[3].parse::<i8>().unwrap()
        });
    }

    (stacks, moves)
}

fn print_stacks(stacks: &HashMap<i8, Stack>) {
    let mut lines: Vec<String> = vec![];
    let max_nb_crates = stacks.values().map(|s| s.crates.borrow().len()).max().unwrap();

    for i in 0..max_nb_crates {
        let mut line = String::new();
        for n in 0..stacks.len() {
            let crates = stacks.get(&((n as i8) + 1)).unwrap().crates.borrow();
            if i < crates.len() {
                line.push_str(&format!("[{}] ", crates[i]));
            }
            else {
                line.push_str("    ");
            }
        }
        lines.push(line.clone());
    }

    let separator = (0..stacks.len() * 4).map(|_| "-").collect::<String>();
    let stack_ids_line = stacks.keys().sorted().map(|key| format!(" {}  ", key)).collect::<String>();

    println!("");
    for l in lines.iter().rev() {
        println!("{l}");
    }
    println!("{separator}");
    println!("{stack_ids_line}");
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
        assert_eq!("CMZ", solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!("MCD", solution2(data));
    }
}
