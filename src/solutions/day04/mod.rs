use regex::Regex;
use std::collections::HashSet;

pub fn solution1(data: String) -> i32 {
    let result = data
        .lines()
        .map(|line| get_ranges(&line))
        .map(|ranges| ranges_to_sets(ranges))
        .map(|sets| {
            return if sets[0].is_subset(&sets[1]) || sets[1].is_subset(&sets[0]) {
                1
            }
            else {
                0
            };
        })
        .sum();

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> i32 {
    let result = data
        .lines()
        .map(|line| get_ranges(&line))
        .map(|ranges| ranges_to_sets(ranges))
        .map(|sets| {
            let intersection: HashSet<&i32> = sets[0].intersection(&sets[1]).collect();
            return if intersection.len() > 0 {
                1
            }
            else {
                0
            };
        })
        .sum();

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

struct Range {
    min: i32,
    max: i32
}

fn get_ranges(line: &str) -> Vec<Range> {
    let ranges_regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let captures = ranges_regex.captures(line).unwrap();

    let r1 = Range { min: captures[1].parse::<i32>().unwrap(), max: captures[2].parse::<i32>().unwrap() };
    let r2 = Range { min: captures[3].parse::<i32>().unwrap(), max: captures[4].parse::<i32>().unwrap() };
    vec![r1, r2]
}

fn ranges_to_sets(ranges: Vec<Range>) -> Vec<HashSet<i32>> {
    ranges.iter().map(|r| HashSet::from_iter(r.min..r.max + 1)).collect()
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
        assert_eq!(2, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(4, solution2(data));
    }
}
