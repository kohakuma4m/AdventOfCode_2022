use crate::navigation::{Coordinate, Direction, Grid};
use itertools::Itertools;

pub fn solution1(data: String) -> usize {
    let moves = read_moves(data);

    let mut head_location = Coordinate { x: 0, y: 0 };
    let mut tail_location = Coordinate { x: 0, y: 0 };
    let mut tail_grid = Grid::new();
    tail_grid.add_location(Coordinate { x: 0, y: 0 }, Symbol::Start);

    for m in moves {
        for _ in 0..m.nb_steps {
            let next_head_location = get_next_head_knot_location(&head_location, &m.direction);
            tail_location = get_next_tail_knot_location(&next_head_location, &tail_location);
            head_location = next_head_location;

            // Mapping tail location
            if tail_location != (Coordinate { x: 0, y: 0 }) {
                tail_grid.add_location(tail_location.clone(), Symbol::Visited);
            }
        }
    }

    tail_grid.print(&Symbol::Empty, &symbol_to_char);

    let result = 1 + tail_grid.count_values(&Symbol::Visited);

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> usize {
    let moves = read_moves(data);

    let mut knot_locations: Vec<Coordinate<isize>> = (0..10).map(|_| Coordinate { x: 0, y: 0 }).collect();
    let mut tail_grid = Grid::new();
    tail_grid.add_location(Coordinate { x: 0, y: 0 }, Symbol::Start);

    for m in moves {
        for _ in 0..m.nb_steps {
            // Moving head knot
            knot_locations[0] = get_next_head_knot_location(&knot_locations[0], &m.direction);

            // Moving all following knots
            for i in 1..knot_locations.len() {
                knot_locations[i] = get_next_tail_knot_location(&knot_locations[i - 1], &knot_locations[i]);
            }

            // Mapping tail location
            let tail_location = knot_locations.last().unwrap();
            if tail_location != &(Coordinate { x: 0, y: 0 }) {
                tail_grid.add_location(tail_location.clone(), Symbol::Visited);
            }
        }
    }

    tail_grid.print(&Symbol::Empty, &symbol_to_char);

    let result = 1 + tail_grid.count_values(&Symbol::Visited);

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

struct Move {
    direction: Direction,
    nb_steps: i32
}

pub fn str_to_direction(d: &str) -> Direction {
    match d {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        _ => panic!("Invalid direction {d}")
    }
}

fn read_moves(data: String) -> Vec<Move> {
    data.lines()
        .map(|l| {
            let (d, n) = l.split_whitespace().collect_tuple().unwrap();
            return Move { direction: str_to_direction(d), nb_steps: n.parse::<i32>().unwrap() };
        })
        .collect()
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Symbol {
    Start,
    Visited,
    Empty
}

fn symbol_to_char(symbol: &Symbol) -> &str {
    match symbol {
        Symbol::Start => "s",
        Symbol::Visited => "#",
        Symbol::Empty => "."
    }
}

fn get_next_head_knot_location(head_knot_location: &Coordinate<isize>, direction: &Direction) -> Coordinate<isize> {
    match direction {
        Direction::Up => Coordinate { x: head_knot_location.x, y: head_knot_location.y - 1 },
        Direction::Right => Coordinate { x: head_knot_location.x + 1, y: head_knot_location.y },
        Direction::Down => Coordinate { x: head_knot_location.x, y: head_knot_location.y + 1 },
        Direction::Left => Coordinate { x: head_knot_location.x - 1, y: head_knot_location.y }
    }
}

fn get_next_tail_knot_location(head_knot_location: &Coordinate<isize>, tail_knot_location: &Coordinate<isize>) -> Coordinate<isize> {
    let mut dx = head_knot_location.x - tail_knot_location.x;
    let mut dy = head_knot_location.y - tail_knot_location.y;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        // Tail knot is already touching head knot, so no moving required...
        return tail_knot_location.clone();
    }

    // Tail knot must move to previous head knot location to keep up...
    if dx.abs() > 1 {
        dx = if dx > 0 {
            dx - 1
        }
        else {
            dx + 1
        };
    }
    if dy.abs() > 1 {
        dy = if dy > 0 {
            dy - 1
        }
        else {
            dy + 1
        };
    }
    Coordinate { x: tail_knot_location.x + dx, y: tail_knot_location.y + dy }
}

/////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_file(filename: &str) -> String {
        let current_file = std::file!();
        let test_file = current_file.replace("mod.rs", filename);
        return crate::read_file(&test_file).unwrap();
    }

    #[test]
    fn test_solution1() {
        let data = read_test_file("test.txt");
        assert_eq!(13, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file("test2.txt");
        assert_eq!(36, solution2(data));
    }
}
