use itertools::Itertools;
use std::collections::HashSet;

pub fn solution1(data: String) -> u32 {
    let result = data
        .lines()
        .map(|line| get_rucksack_misplaced_items(line))
        .map(|items| {
            // General case if there is more than one misplaced items
            let partial_sum: u32 = items.iter().map(|c| get_letter_value(c)).sum();
            partial_sum
        })
        .sum();

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> u32 {
    let lines: Vec<&str> = data.lines().collect();

    let mut items: Vec<char> = vec![];
    for chunk in &lines.into_iter().chunks(3) {
        let group: Vec<&str> = chunk.collect();
        for item in get_common_items_for_each_group(group) {
            items.push(item); // Should be only one badge per group, but general case...
        }
    }

    let result: u32 = items.iter().map(|c| get_letter_value(c)).sum();

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

fn get_letter_value(c: &char) -> u32 {
    match c {
        'a'..='z' => (*c as u32) - 96,      // 1-26
        'A'..='Z' => (*c as u32) - 64 + 26, // 27-52
        _ => panic!("Invalid char {c}")
    }
}

fn get_rucksack_misplaced_items(line: &str) -> Vec<char> {
    let (left, right) = line.split_at(line.len() / 2);
    let set1: HashSet<char> = HashSet::from_iter(left.chars());
    let set2: HashSet<char> = HashSet::from_iter(right.chars());
    let common_items: Vec<char> = set1.intersection(&set2).map(|c| *c).collect();
    common_items
}

fn get_common_items_for_each_group(group: Vec<&str>) -> Vec<char> {
    let set1: HashSet<char> = HashSet::from_iter(group[0].chars());
    let set2: HashSet<char> = HashSet::from_iter(group[1].chars());
    let set3: HashSet<char> = HashSet::from_iter(group[2].chars());
    let set12: HashSet<char> = set1.intersection(&set2).copied().collect();
    let common_items: Vec<char> = set12.intersection(&set3).map(|c| *c).sorted().collect();
    common_items
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
        assert_eq!(157, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(70, solution2(data));
    }
}
