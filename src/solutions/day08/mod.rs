use crate::navigation::Coordinate;
use std::collections::HashMap;

pub fn solution1(data: String) -> usize {
    let trees_map = read_tree_map(data);
    let result = trees_map.count_visible_trees();

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> i32 {
    let trees_map = read_tree_map(data);
    let result = trees_map.find_best_scenic_score();

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug)]
struct Tree {
    location: Coordinate<usize>,
    height: u8
}

impl Tree {
    fn is_visible(&self, trees_map: &TreesMap) -> bool {
        if self.location.x == 0 || self.location.x == trees_map.width - 1 || self.location.y == 0 || self.location.y == trees_map.height - 1 {
            return true; // Trees on the edge are all visibles
        }

        // Checking rows
        if (0..self.location.x).all(|x| trees_map.get_tree(Coordinate { x, y: self.location.y }).unwrap().height < self.height) {
            return true; // Tree is visible from the left
        }
        if (self.location.x + 1..trees_map.width).all(|x| trees_map.get_tree(Coordinate { x, y: self.location.y }).unwrap().height < self.height) {
            return true; // Tree is visible from the right
        }

        // Checking columns
        if (0..self.location.y).all(|y| trees_map.get_tree(Coordinate { x: self.location.x, y }).unwrap().height < self.height) {
            return true; // Tree is visible from the top
        }
        if (self.location.y + 1..trees_map.height).all(|y| trees_map.get_tree(Coordinate { x: self.location.x, y }).unwrap().height < self.height) {
            return true; // Tree is visible from the bottom
        }

        false // Tree is not visible from any side
    }

    fn calculate_scenic_score(&self, trees_map: &TreesMap) -> i32 {
        // Looking left
        let mut left_viewing_distance = 0;
        for x in (0..self.location.x).rev() {
            left_viewing_distance += 1;

            if trees_map.get_tree(Coordinate { x, y: self.location.y }).unwrap().height >= self.height {
                break;
            }
        }

        // Looking right
        let mut right_viewing_distance = 0;
        for x in self.location.x + 1..trees_map.width {
            right_viewing_distance += 1;

            if trees_map.get_tree(Coordinate { x, y: self.location.y }).unwrap().height >= self.height {
                break;
            }
        }

        // Looking up
        let mut top_viewing_distance = 0;
        for y in (0..self.location.y).rev() {
            top_viewing_distance += 1;

            if trees_map.get_tree(Coordinate { x: self.location.x, y }).unwrap().height >= self.height {
                break;
            }
        }

        // Looking down
        let mut bottom_viewing_distance = 0;
        for y in self.location.y + 1..trees_map.height {
            bottom_viewing_distance += 1;

            if trees_map.get_tree(Coordinate { x: self.location.x, y }).unwrap().height >= self.height {
                break;
            }
        }

        left_viewing_distance * right_viewing_distance * top_viewing_distance * bottom_viewing_distance
    }
}

struct TreesMap {
    height: usize,
    width: usize,
    trees: HashMap<Coordinate<usize>, Tree>
}

impl TreesMap {
    fn get_tree(&self, location: Coordinate<usize>) -> Option<&Tree> {
        self.trees.get(&location)
    }

    fn count_visible_trees(&self) -> usize {
        self.trees.values().filter(|t| t.is_visible(&self)).count()
    }

    fn find_best_scenic_score(&self) -> i32 {
        self.trees.values().map(|t| t.calculate_scenic_score(&self)).max().unwrap()
    }
}

fn read_tree_map(data: String) -> TreesMap {
    let mut trees: HashMap<Coordinate<usize>, Tree> = HashMap::new();

    let mut height = 0;
    let mut width = 0;
    data.lines().enumerate().for_each(|(y, line)| {
        height = y + 1;
        width = line.len();
        line.chars().enumerate().for_each(|(x, height)| {
            let tree = Tree { location: Coordinate { x, y }, height: height.to_digit(10).unwrap() as u8 };
            trees.insert(tree.location, tree);
        })
    });

    TreesMap { height, width, trees }
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
        assert_eq!(21, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(8, solution2(data));
    }
}
