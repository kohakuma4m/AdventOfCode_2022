use std::collections::HashMap;

use regex::Regex;

use crate::navigation::{get_adjacent_locations_in_direction, get_direction_after_rotation, Coordinate, Direction, Grid, Rotation};

pub fn solution1(data: String) -> isize {
    let (start, path_instructions, mut board_map) = read_board_map_data(data);
    board_map.print(&Symbol::Void, &symbol_to_char);

    // TODO: mark path as we go to print later...
    let (end, direction) = find_end_location(&start, &path_instructions, &mut board_map);

    let result = get_password(&end, &direction);

    println!("=========================");
    println!("Start: {:?} --> {:?}", start, Direction::Right);
    println!("End  : {:?} --> {:?}", end, direction);
    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> isize {
    let (start, path_instructions, mut board_map) = read_board_map_data(data);
    board_map.print(&Symbol::Void, &symbol_to_char);

    let folding_map = map_folding_border_locations(&board_map);

    // TODO: mark path as we go to print later...
    let (end, direction) = find_end_location(&start, &path_instructions, &mut board_map);

    let result = get_password(&end, &direction);

    println!("=========================");
    println!("Start: {:?} --> {:?}", start, Direction::Right);
    println!("End  : {:?} --> {:?}", end, direction);
    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Hash)]
enum Symbol {
    Void,
    Empty,
    Wall
}

fn symbol_to_char(symbol: &Symbol) -> &str {
    match symbol {
        Symbol::Void => " ",
        Symbol::Empty => ".",
        Symbol::Wall => "#"
    }
}

#[derive(Debug)]
enum PathInstruction {
    Move(usize),
    Rotate(Rotation)
}

fn read_board_map_data(data: String) -> (Coordinate<isize>, Vec<PathInstruction>, Grid<isize, Symbol>) {
    let mut start: Coordinate<isize> = Coordinate { x: 0, y: 0 };
    let mut path_instructions = vec![];
    let mut board_map = Grid::new();

    let path_description_regex = Regex::new(r"(\d+|R|L)").unwrap();

    let mut y = 0;
    let mut lines_iterator = data.lines();
    while let Some(line) = lines_iterator.next() {
        if line == "" {
            // Last path description line
            let line = lines_iterator.next().unwrap();
            for capture in path_description_regex.captures_iter(line) {
                match &capture[1] {
                    "R" => path_instructions.push(PathInstruction::Rotate(Rotation::Clockwise)),
                    "L" => path_instructions.push(PathInstruction::Rotate(Rotation::CounterClockwise)),
                    _ => path_instructions.push(PathInstruction::Move(capture[1].parse::<usize>().unwrap()))
                }
            }
            break;
        }

        // Board map
        line.chars().enumerate().for_each(|(x, char)| {
            if char == '.' {
                if y == 0 && start.x == 0 {
                    start = Coordinate { x: x as isize, y }
                }
                board_map.add_location(Coordinate { x: x as isize, y }, Symbol::Empty);
            }
            else if char == '#' {
                board_map.add_location(Coordinate { x: x as isize, y }, Symbol::Wall);
            }
        });

        y += 1;
    }

    (start, path_instructions, board_map)
}

fn get_password(location: &Coordinate<isize>, direction: &Direction) -> isize {
    let direction_value = match direction {
        Direction::Up => 3,
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2
    };

    1000 * (location.y + 1) + 4 * (location.x + 1) + direction_value
}

fn find_end_location(start: &Coordinate<isize>, instructions: &Vec<PathInstruction>, map: &mut Grid<isize, Symbol>) -> (Coordinate<isize>, Direction) {
    let mut direction = Direction::Right;
    let mut current = start.clone();

    for instruction in instructions.iter() {
        match instruction {
            PathInstruction::Move(nb_steps) => {
                // Moving forward n steps
                for _ in 0..*nb_steps {
                    let next_location = get_adjacent_locations_in_direction(&current, &direction);
                    current = match map.get_value(&next_location) {
                        Some(Symbol::Wall) => break,
                        Some(Symbol::Empty) => next_location,
                        _ => {
                            // Looping around to other side of map
                            let looping_location = find_looping_location(&current, &direction, &map);
                            match map.get_value(&looping_location) {
                                Some(Symbol::Wall) => break,
                                Some(Symbol::Empty) => looping_location,
                                _ => panic!("Looping into void {:?} {:?}--> {:?}", current, direction, looping_location)
                            }
                        }
                    }
                }
            },
            PathInstruction::Rotate(rotation) => {
                // Turning in place
                direction = get_direction_after_rotation(&direction, rotation);
            }
        }
    }

    (current, direction)
}

fn find_looping_location(location: &Coordinate<isize>, direction: &Direction, map: &Grid<isize, Symbol>) -> Coordinate<isize> {
    match direction {
        Direction::Up => {
            // Looping to the bottom
            let mut y = map.max_y();
            while map.get_value(&Coordinate { x: location.x, y }) == None {
                y -= 1;
            }
            Coordinate { x: location.x, y }
        },
        Direction::Right => {
            // Looping to the left
            let mut x = 0;
            while map.get_value(&Coordinate { x, y: location.y }) == None {
                x += 1;
            }
            Coordinate { x, y: location.y }
        },
        Direction::Down => {
            // Looping to the top
            let mut y = 0;
            while map.get_value(&Coordinate { x: location.x, y }) == None {
                y += 1;
            }
            Coordinate { x: location.x, y }
        },
        Direction::Left => {
            // Looping to the right
            let mut x = map.max_x();
            while map.get_value(&Coordinate { x, y: location.y }) == None {
                x -= 1;
            }
            Coordinate { x, y: location.y }
        }
    }
}

#[derive(Debug)]
struct CubeFace {
    id: usize,
    top_left: Coordinate<isize>,
    size: isize,
    top: usize,
    right: usize,
    down: usize,
    left: usize
}

impl CubeFace {
    fn new(id: usize, size: isize, top_left: Coordinate<isize>) -> CubeFace {
        CubeFace { id, size, top_left, top: 0, right: 0, down: 0, left: 0 }
    }

    fn min_x(&self) -> isize {
        self.top_left.x
    }

    fn max_x(&self) -> isize {
        self.top_left.x + self.size
    }

    fn min_y(&self) -> isize {
        self.top_left.y
    }

    fn max_y(&self) -> isize {
        self.top_left.y + self.size
    }
}

fn map_folding_border_locations(map: &Grid<isize, Symbol>) -> HashMap<Coordinate<isize>, (Coordinate<isize>, Direction)> {
    let folding_map = HashMap::new();

    // 1) Getting each cube face size length (There is 11 unique cube nets)
    let max_x = map.max_x();
    let max_y = map.max_y();
    let mut face_size = (max_y - max_x).abs();
    if face_size == max_x || face_size == max_y {
        face_size /= 2; // To account for 11th cube net not following pattern 3/4 dimension on paper...
    }

    // 2) Mapping faces
    let mut x = 0;
    let mut y = 0;
    let mut faces = vec![];
    while y < max_y {
        while x < max_x {
            if map.get_value(&Coordinate { x, y }) != None {
                faces.push(CubeFace::new(faces.len() + 1, face_size, Coordinate { x, y }));
            }
            x += face_size;
        }

        x = 0;
        y += face_size;
    }

    // 3) Linking all face edges ???
    for f in faces {}

    // 4) Linking all face edge adjacent locations together ???

    folding_map
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
        assert_eq!(6032, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(5031, solution2(data));
    }
}
