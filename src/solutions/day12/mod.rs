use crate::navigation::{find_shortest_path, Coordinate, Grid, PathTarget};

pub fn solution1(data: String) -> usize {
    let (start, goal, elevation_map) = read_elevation_map(data);
    elevation_map.print(&Symbol::Empty, &symbol_to_char);
    println!("Start: {:?}", start);
    println!("Goal: {:?}", goal);

    // Finding shortest path from start to goal
    let location_validator = build_location_validator(false);
    let path = match find_shortest_path(&elevation_map, &start, &PathTarget::Location(goal), Some(location_validator)) {
        Some(path) => path,
        None => panic!("No path found !")
    };
    let result = path.locations.len() - 1; // Excluding starting position

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> usize {
    let (_, goal, elevation_map) = read_elevation_map(data);
    elevation_map.print(&Symbol::Empty, &symbol_to_char);
    println!("Goal: {:?}", goal);

    // Finding reverse shortest path from goal to first location with lowest elevation value "a"
    let location_validator = build_location_validator(true);
    let reverse_path = match find_shortest_path(&elevation_map, &goal, &&PathTarget::Value(Symbol::Elevation("a".to_string())), Some(location_validator)) {
        Some(path) => path,
        None => panic!("No path found !")
    };
    let result = reverse_path.locations.len() - 1; // Excluding starting position

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Hash)]
enum Symbol {
    Start,
    Goal,
    Visited(String),   // Direction
    Elevation(String), // Elevation
    Empty
}

fn symbol_to_char(symbol: &Symbol) -> &str {
    match symbol {
        Symbol::Start => "S",
        Symbol::Goal => "E",
        Symbol::Visited(direction) => direction,
        Symbol::Elevation(elevation) => elevation,
        Symbol::Empty => "."
    }
}

fn read_elevation_map(data: String) -> (Coordinate<isize>, Coordinate<isize>, Grid<isize, Symbol>) {
    // TODO: usize instead ???
    let mut start: Coordinate<isize> = Coordinate { x: 0, y: 0 };
    let mut goal: Coordinate<isize> = Coordinate { x: 0, y: 0 };
    let mut elevation_map = Grid::new();

    data.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, char)| {
            let value = match char {
                'S' => {
                    start = Coordinate { x: x as isize, y: y as isize };
                    Symbol::Start
                },
                'E' => {
                    goal = Coordinate { x: x as isize, y: y as isize };
                    Symbol::Goal
                },
                elevation => Symbol::Elevation(elevation.to_string())
            };

            elevation_map.add_location(Coordinate { x: x as isize, y: y as isize }, value);
        })
    });

    (start, goal, elevation_map)
}

fn build_location_validator(is_reverse: bool) -> Box<dyn Fn(&Symbol, &Symbol) -> bool> {
    Box::new(move |current_elevation: &Symbol, new_elevation: &Symbol| -> bool {
        let e1 = match current_elevation {
            Symbol::Start => 'a' as u32,
            Symbol::Goal => 'z' as u32,
            Symbol::Elevation(e) => e.chars().next().unwrap() as u32,
            _ => panic!("Invalid elevation: {:?}", current_elevation)
        };
        let e2 = match new_elevation {
            Symbol::Start => 'a' as u32,
            Symbol::Goal => 'z' as u32,
            Symbol::Elevation(e) => e.chars().next().unwrap() as u32,
            _ => panic!("Invalid elevation: {:?}", new_elevation)
        };

        if is_reverse {
            e1 < e2 || e1 - e2 <= 1
        }
        else {
            e2 < e1 || e2 - e1 <= 1
        }
    })
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
        assert_eq!(31, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(29, solution2(data));
    }
}
