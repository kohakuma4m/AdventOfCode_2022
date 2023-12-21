use crate::navigation::{
    get_adjacent_diagonal_locations, get_adjacent_locations_in_cardinal_direction, get_adjacent_orthogonal_locations, CardinalDirection, Coordinate, Direction,
    Grid
};
use std::collections::HashMap;

pub fn solution1(data: String) -> usize {
    let mut grove_map = read_grove_map(data);

    simulate_rounds(&mut grove_map, Some(10));

    // Trimming map to remove extra empty locations on border
    grove_map.keep_only_matching_locations(&|_, value| -> bool { value == &Symbol::Elf });

    let result = (grove_map.width() * grove_map.height()) as usize - grove_map.get_mapped_locations_with_value(&Symbol::Elf).len();

    println!("=========================");
    grove_map.print(&Symbol::Empty, &symbol_to_char);
    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> usize {
    let mut grove_map = read_grove_map(data);

    let result = simulate_rounds(&mut grove_map, None);

    println!("=========================");
    grove_map.print(&Symbol::Empty, &symbol_to_char);
    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Hash)]
enum Symbol {
    Elf,
    Empty
}

fn symbol_to_char(symbol: &Symbol) -> &str {
    match symbol {
        Symbol::Elf => "#",
        Symbol::Empty => "."
    }
}

fn read_grove_map(data: String) -> Grid<isize, Symbol> {
    let mut grove_map = Grid::new();

    data.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            let value = match char {
                '.' => Symbol::Empty,
                '#' => Symbol::Elf,
                _ => panic!("Invalid char {char}")
            };

            grove_map.add_location(Coordinate { x: x as isize, y: y as isize }, value);
        });
    });

    grove_map
}

fn simulate_rounds(grove_map: &mut Grid<isize, Symbol>, max_nb_rounds: Option<usize>) -> usize {
    let valid_directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    let mut round_idx = 0;
    while max_nb_rounds == None || round_idx < max_nb_rounds.unwrap() {
        // Beginning of round
        if round_idx % 10 == 0 {
            println!("Round #{}", round_idx + 1);
        }

        // 1) Getting all proposed new locations
        let mut new_locations_map: HashMap<Coordinate<isize>, Vec<Coordinate<isize>>> = HashMap::new();
        for elf_location in grove_map.get_locations_with_value(&Symbol::Elf).into_iter() {
            let adjacent_locations = [
                get_adjacent_orthogonal_locations(&elf_location), // North, East, South, West
                get_adjacent_diagonal_locations(&elf_location)    // NE, SE, SW, NW
            ]
            .concat();
            let adjacent_values: Vec<Option<&Symbol>> = adjacent_locations.iter().map(|l| grove_map.get_value(&l)).collect();
            if adjacent_values.iter().all(|v| v.unwrap_or(&Symbol::Empty) == &Symbol::Empty) {
                continue; // Elf does not move
            }

            // Choosing direction in order based on round number
            for i in round_idx..round_idx + 4 {
                let direction = &valid_directions[i % 4];
                match direction {
                    Direction::Up => {
                        // North, NE, NW
                        if [&adjacent_values[0], &adjacent_values[4], &adjacent_values[7]].iter().all(|v| v.unwrap_or(&Symbol::Empty) == &Symbol::Empty) {
                            let new_location = get_adjacent_locations_in_cardinal_direction(&elf_location, &CardinalDirection::North);
                            if new_locations_map.contains_key(&new_location) == false {
                                new_locations_map.insert(new_location.clone(), vec![]);
                            }

                            new_locations_map.get_mut(&new_location).unwrap().push(elf_location);
                            break;
                        }
                    },
                    Direction::Right => {
                        // East, NE, SE
                        if [&adjacent_values[1], &adjacent_values[4], &adjacent_values[5]].iter().all(|v| v.unwrap_or(&Symbol::Empty) == &Symbol::Empty) {
                            let new_location = get_adjacent_locations_in_cardinal_direction(&elf_location, &CardinalDirection::East);
                            if new_locations_map.contains_key(&new_location) == false {
                                new_locations_map.insert(new_location.clone(), vec![]);
                            }

                            new_locations_map.get_mut(&new_location).unwrap().push(elf_location);
                            break;
                        }
                    },
                    Direction::Down => {
                        // South, SE, SW
                        if [&adjacent_values[2], &adjacent_values[5], &adjacent_values[6]].iter().all(|v| v.unwrap_or(&Symbol::Empty) == &Symbol::Empty) {
                            let new_location = get_adjacent_locations_in_cardinal_direction(&elf_location, &CardinalDirection::South);
                            if new_locations_map.contains_key(&new_location) == false {
                                new_locations_map.insert(new_location.clone(), vec![]);
                            }

                            new_locations_map.get_mut(&new_location).unwrap().push(elf_location);
                            break;
                        }
                    },
                    Direction::Left => {
                        // West, SW, NW
                        if [&adjacent_values[3], &adjacent_values[6], &adjacent_values[7]].iter().all(|v| v.unwrap_or(&Symbol::Empty) == &Symbol::Empty) {
                            let new_location = get_adjacent_locations_in_cardinal_direction(&elf_location, &CardinalDirection::West);
                            if new_locations_map.contains_key(&new_location) == false {
                                new_locations_map.insert(new_location.clone(), vec![]);
                            }

                            new_locations_map.get_mut(&new_location).unwrap().push(elf_location);
                            break;
                        }
                    }
                }
            }
        }

        if new_locations_map.len() == 0 {
            println!("End of process");
            break; // End of process
        }

        // 2) Moving elfs
        new_locations_map
            .into_iter()
            // Moving elf to new location only no other elves proposed moving there
            .filter(|(_, elf_locations)| elf_locations.len() < 2)
            .for_each(|(new_location, elf_locations)| {
                grove_map.add_location(elf_locations[0], Symbol::Empty);
                grove_map.add_location(new_location, Symbol::Elf);
            });

        // End of round
        round_idx += 1;
    }

    round_idx + 1
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
        assert_eq!(110, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(20, solution2(data));
    }
}
