use std::collections::{HashMap, HashSet};

use crate::navigation::{get_adjacent_locations_in_direction, get_adjacent_orthogonal_locations, Coordinate, Direction, Grid, Path};

pub fn solution1(data: String) -> usize {
    let (start, goal, mut valley_map) = read_valley_map(data);
    let mut blizzard_locations_map = map_blizzard_locations(&valley_map);

    println!("Start: {:?} --> Goal: {:?}", start, goal);
    valley_map.print(&Symbol::Empty, &symbol_to_char);

    let path = match find_shortest_path(&start, &goal, &mut valley_map, &mut blizzard_locations_map) {
        Some(path) => path,
        None => panic!("No path found !")
    };

    valley_map.print(&Symbol::Empty, &symbol_to_char);

    let result = path.locations.len() - 1; // Excluding starting position

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> usize {
    let (start, goal, mut valley_map) = read_valley_map(data);
    let mut blizzard_locations_map = map_blizzard_locations(&valley_map);

    println!("Start: {:?} --> Goal: {:?}", start, goal);
    valley_map.print(&Symbol::Empty, &symbol_to_char);

    // First trip
    let path1 = match find_shortest_path(&start, &goal, &mut valley_map, &mut blizzard_locations_map) {
        Some(path) => path,
        None => panic!("No path found !")
    };
    valley_map.print(&Symbol::Empty, &symbol_to_char);

    // Going back for snack
    let path2 = match find_shortest_path(&goal, &start, &mut valley_map, &mut blizzard_locations_map) {
        Some(path) => path,
        None => panic!("No path found !")
    };
    valley_map.print(&Symbol::Empty, &symbol_to_char);

    // Final trip
    let path3 = match find_shortest_path(&start, &goal, &mut valley_map, &mut blizzard_locations_map) {
        Some(path) => path,
        None => panic!("No path found !")
    };
    valley_map.print(&Symbol::Empty, &symbol_to_char);

    let result = path1.locations.len() + path2.locations.len() + path3.locations.len() - 3; // Excluding starting positions

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Symbol {
    Wall,
    Empty,
    Blizzard(Direction),
    MultiBlizzard(u8)
}

fn symbol_to_char(symbol: &Symbol) -> &str {
    match symbol {
        Symbol::Wall => "#",
        Symbol::Empty => ".",
        Symbol::Blizzard(direction) => match direction {
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "v",
            Direction::Left => "<"
        },
        Symbol::MultiBlizzard(_) => "*" // TODO: refractor to return string instead to return number ???
    }
}

fn read_valley_map(data: String) -> (Coordinate<isize>, Coordinate<isize>, Grid<isize, Symbol>) {
    let mut start: Coordinate<isize> = Coordinate { x: 0, y: 0 };
    let mut goal: Coordinate<isize> = Coordinate { x: 0, y: 0 };
    let mut valley_map = Grid::new();

    let nb_lines = data.lines().count();
    data.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            let value = match char {
                '.' => {
                    if y == 0 {
                        start = Coordinate { x: x as isize, y: y as isize };
                    }

                    if y == nb_lines - 1 {
                        goal = Coordinate { x: x as isize, y: y as isize };
                    }

                    Symbol::Empty
                },
                '#' => Symbol::Wall,
                '^' => Symbol::Blizzard(Direction::Up),
                '>' => Symbol::Blizzard(Direction::Right),
                '<' => Symbol::Blizzard(Direction::Left),
                'v' => Symbol::Blizzard(Direction::Down),
                _ => panic!("Invalid char {char}")
            };

            valley_map.add_location(Coordinate { x: x as isize, y: y as isize }, value);
        });
    });

    (start, goal, valley_map)
}

fn map_blizzard_locations(map: &Grid<isize, Symbol>) -> HashMap<Coordinate<isize>, Vec<Symbol>> {
    let blizzard_locations = map.get_locations_with_values(vec![
        &Symbol::Blizzard(Direction::Up),
        &Symbol::Blizzard(Direction::Right),
        &Symbol::Blizzard(Direction::Left),
        &Symbol::Blizzard(Direction::Down),
    ]);

    let mut blizzard_locations_map: HashMap<Coordinate<isize>, Vec<Symbol>> = HashMap::new();
    for location in blizzard_locations.into_iter() {
        if blizzard_locations_map.contains_key(&location) == false {
            blizzard_locations_map.insert(location.clone(), vec![]);
        }
        let value = match map.get_value(&location).unwrap() {
            Symbol::Blizzard(Direction::Up) => Symbol::Blizzard(Direction::Up),
            Symbol::Blizzard(Direction::Right) => Symbol::Blizzard(Direction::Right),
            Symbol::Blizzard(Direction::Left) => Symbol::Blizzard(Direction::Left),
            Symbol::Blizzard(Direction::Down) => Symbol::Blizzard(Direction::Down),
            _ => panic!("Non blizzard value...")
        };
        blizzard_locations_map.get_mut(&location).unwrap().push(value);
    }

    blizzard_locations_map
}

fn find_shortest_path(
    start: &Coordinate<isize>,
    goal: &Coordinate<isize>,
    map: &mut Grid<isize, Symbol>,
    blizzard_locations_map: &mut HashMap<Coordinate<isize>, Vec<Symbol>>
) -> Option<Path<isize>> {
    let mut visited_locations: HashSet<(Coordinate<isize>, usize)> = HashSet::new();
    let mut paths_to_explored: Vec<Path<isize>> = vec![Path::new(start)];

    let mut path_size = 0;
    let mut distance_to_goal;
    while paths_to_explored.len() > 0 {
        path_size += 1;
        distance_to_goal = paths_to_explored.iter().map(|p| p.locations.last().unwrap().manhattan_distance(goal)).min().unwrap();
        println!("Path size: {:3}; distance to goal {:3} --> nb paths to explore {}", path_size, distance_to_goal, paths_to_explored.len());

        // Updating blizzard locations
        let blizzard_locations: HashMap<Coordinate<isize>, Vec<Symbol>> = blizzard_locations_map.drain().collect();
        for (location, directions) in blizzard_locations.iter() {
            for blizzard in directions.iter() {
                // Clearing previous blizzard location
                map.add_location(location.clone(), Symbol::Empty);

                // Calculating new blizzard location
                match blizzard {
                    Symbol::Blizzard(direction) => {
                        let mut new_location = get_adjacent_locations_in_direction(&location, direction);
                        if &new_location == start || &new_location == goal {
                            // Blizzard warping back to other side of the map
                            new_location = find_looping_location(&new_location, direction, map);
                        }
                        else if let Some(Symbol::Wall) = map.get_value(&new_location) {
                            // Blizzard warping back to other side of the map
                            new_location = find_looping_location(&new_location, direction, map);
                        }

                        if blizzard_locations_map.contains_key(&new_location) == false {
                            blizzard_locations_map.insert(new_location.clone(), vec![]);
                        }
                        blizzard_locations_map.get_mut(&new_location).unwrap().push(blizzard.clone());
                    },
                    _ => {}
                }
            }
        }

        // Updating map
        for (location, directions) in blizzard_locations_map.iter() {
            if directions.len() > 1 {
                map.add_location(location.clone(), Symbol::MultiBlizzard(directions.len() as u8));
            }
            else {
                map.add_location(location.clone(), directions[0].clone());
            }
        }

        let current_paths: Vec<Path<isize>> = paths_to_explored.drain(..).collect();
        for path in current_paths {
            let nb_visited_locations = path.locations.len();
            let current_location = path.locations.last().unwrap();
            let adjacent_locations = get_adjacent_orthogonal_locations(&current_location);
            let mut next_locations: Vec<Coordinate<isize>> = adjacent_locations
                .into_iter()
                // We must allow moving back to previously visited locations to avoid blizzards...
                // ...but we should still prune duplicate path ending at the same location after n steps, since they won't lead to a better solution
                .filter(|l| visited_locations.contains(&(l.clone(), nb_visited_locations + 1)) == false)
                // Removing locations outside map or wall or blizzard location
                .filter(|l| match map.get_value(&l) {
                    None | Some(Symbol::Wall) => false,
                    _ => blizzard_locations_map.contains_key(&l) == false
                })
                .collect();

            if blizzard_locations_map.contains_key(&current_location) == false {
                // Waiting for blizzard to clear (non moving step)
                next_locations.push(current_location.clone());
            }

            for location in next_locations.into_iter() {
                let mut next_path = Path::from(&path);
                next_path.locations.push(location.clone());

                if &location == goal {
                    // Found one shortest path to goal
                    return Some(next_path);
                }

                visited_locations.insert((location, next_path.locations.len()));
                paths_to_explored.push(next_path);
            }
        }
    }

    None
}

fn find_looping_location(location: &Coordinate<isize>, direction: &Direction, map: &Grid<isize, Symbol>) -> Coordinate<isize> {
    match direction {
        Direction::Up => {
            // Looping to the bottom
            let mut y = map.max_y();
            while map.get_value(&Coordinate { x: location.x, y }).unwrap() == &Symbol::Wall {
                y -= 1;
            }
            Coordinate { x: location.x, y }
        },
        Direction::Right => {
            // Looping to the left
            let mut x = 0;
            while map.get_value(&Coordinate { x, y: location.y }).unwrap() == &Symbol::Wall {
                x += 1;
            }
            Coordinate { x, y: location.y }
        },
        Direction::Down => {
            // Looping to the top
            let mut y = 0;
            while map.get_value(&Coordinate { x: location.x, y }).unwrap() == &Symbol::Wall {
                y += 1;
            }
            Coordinate { x: location.x, y }
        },
        Direction::Left => {
            // Looping to the right
            let mut x = map.max_x();
            while map.get_value(&Coordinate { x, y: location.y }).unwrap() == &Symbol::Wall {
                x -= 1;
            }
            Coordinate { x, y: location.y }
        }
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
        assert_eq!(18, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(54, solution2(data));
    }
}
