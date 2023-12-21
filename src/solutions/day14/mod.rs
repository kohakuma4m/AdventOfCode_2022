use crate::navigation::{Coordinate, Grid};
use itertools::Itertools;

pub fn solution1(data: String) -> usize {
    let mut cave_map = read_cave_map(data);
    simulate_sand_flow(&mut cave_map, None);

    cave_map.print(&Symbol::Empty, &symbol_to_char);

    let result = cave_map.get_mapped_locations_with_value(&Symbol::Sand(false)).len();

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> usize {
    let mut cave_map = read_cave_map(data);

    let y_floor = cave_map.max_y() + 2;
    simulate_sand_flow(&mut cave_map, Some(y_floor));

    cave_map.print(&Symbol::Empty, &symbol_to_char);

    let result = cave_map.get_mapped_locations_with_value(&Symbol::Sand(false)).len();

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Hash)]
enum Symbol {
    Empty,
    Rock,
    Source,
    Sand(bool) // Flowing or not
}

fn symbol_to_char(symbol: &Symbol) -> &str {
    match symbol {
        Symbol::Empty => ".",
        Symbol::Rock => "#",
        Symbol::Source => "+",
        Symbol::Sand(false) => "o",
        Symbol::Sand(true) => "~"
    }
}

fn read_cave_map(data: String) -> Grid<isize, Symbol> {
    let mut cave_map: Grid<isize, Symbol> = Grid::new();

    // Adding sand source
    cave_map.add_location(Coordinate { x: 500, y: 0 }, Symbol::Source);

    // Adding all rocks
    data.lines().for_each(|line| {
        let coordinates: Vec<Coordinate<isize>> = line
            .split(" -> ")
            .map(|s| {
                let (x, y) = s.split(",").map(|n| n.parse::<isize>().unwrap()).collect_tuple().unwrap();
                Coordinate { x, y }
            })
            .collect();

        for idx in 0..coordinates.len() - 1 {
            let c1 = coordinates[idx];
            let c2 = coordinates[idx + 1];

            if c1.y == c2.y {
                // Horizontal line
                let x_values = if c1.x > c2.x {
                    c2.x..c1.x + 1
                }
                else {
                    c1.x..c2.x + 1
                };
                for x in x_values {
                    cave_map.add_location(Coordinate { x, y: c1.y }, Symbol::Rock);
                }
            }
            else if c1.x == c2.x {
                // Vertical line
                let y_values = if c1.y > c2.y {
                    c2.y..c1.y + 1
                }
                else {
                    c1.y..c2.y + 1
                };
                for y in y_values {
                    cave_map.add_location(Coordinate { x: c1.x, y }, Symbol::Rock);
                }
            }
        }
    });

    cave_map
}

fn simulate_sand_flow(cave_map: &mut Grid<isize, Symbol>, y_floor: Option<isize>) {
    let mut is_flowing_into_the_abyss = false;
    let sand_source = cave_map.get_mapped_locations_with_value(&Symbol::Source)[0];

    let y_max = match y_floor {
        Some(value) => value,    // Solid floor
        None => cave_map.max_y() // Abyss
    };

    loop {
        // New sand unit
        let mut x = sand_source.x;
        let mut y = sand_source.y;

        loop {
            // Adding flowing sand
            if is_flowing_into_the_abyss && y != sand_source.y {
                cave_map.add_location(Coordinate { x, y }, Symbol::Sand(true));
            }

            // Simulating sand unit flow
            match cave_map.get_value(&Coordinate { x, y: y + 1 }) {
                Some(Symbol::Rock) | Some(Symbol::Sand(false)) => {
                    match cave_map.get_value(&Coordinate { x: x - 1, y: y + 1 }) {
                        Some(Symbol::Rock) | Some(Symbol::Sand(false)) => {
                            match cave_map.get_value(&Coordinate { x: x + 1, y: y + 1 }) {
                                Some(Symbol::Rock) | Some(Symbol::Sand(false)) => {
                                    // Resting
                                    cave_map.add_location(Coordinate { x, y }, Symbol::Sand(false));

                                    if y == sand_source.y {
                                        return; // Source is blocked
                                    }

                                    break;
                                },
                                _ => {
                                    x += 1;
                                    y += 1;
                                }
                            }
                        },
                        _ => {
                            // Flowing down-left
                            x -= 1;
                            y += 1;
                        }
                    }
                },
                _ => {
                    // Flowing down
                    y += 1;
                }
            }

            if y_floor != None && y == y_max - 1 {
                // Resting on cave floor
                cave_map.add_location(Coordinate { x, y }, Symbol::Sand(false));
                break;
            }

            if y > y_max {
                // Flowing into the abyss...
                if !is_flowing_into_the_abyss {
                    // Mapping the infinitely flowing sand with an extra loop
                    is_flowing_into_the_abyss = true;
                    break;
                }

                return;
            }
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
        assert_eq!(24, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(93, solution2(data));
    }
}
