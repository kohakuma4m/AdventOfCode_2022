use std::collections::{HashMap, HashSet};

use crate::navigation3d::{find_shortest_path, get_adjacent_orthogonal_locations, Coordinate3D, Grid3D, PathTarget};

use itertools::Itertools;

pub fn solution1(data: String) -> usize {
    let cubes = read_cubes(data);
    let nb_faces = count_non_touching_faces(&cubes);

    println!("=========================");
    println!("Solution1: {nb_faces}");
    println!("=========================");

    nb_faces
}

pub fn solution2(data: String) -> usize {
    let cubes = read_cubes(data);
    let cubes_map = build_cubes_map(&cubes);
    let nb_faces = count_external_faces(&cubes, &cubes_map);

    println!("=========================");
    println!("Solution2: {nb_faces}");
    println!("=========================");

    nb_faces
}

/////////////////////////////////////////////////

fn read_cubes(data: String) -> HashSet<Coordinate3D<isize>> {
    let mut cubes = HashSet::new();

    data.lines().for_each(|line| {
        let (x, y, z) = line.split(",").map(|n| n.parse::<isize>().unwrap()).collect_tuple().unwrap();
        cubes.insert(Coordinate3D { x, y, z });
    });

    cubes
}

fn count_non_touching_faces(cubes: &HashSet<Coordinate3D<isize>>) -> usize {
    let mut nb_faces = 0;

    for c in cubes.iter() {
        // Two unit cubes are touching in one direction if coordinate are adjacent
        for c2 in get_adjacent_orthogonal_locations(&c).into_iter() {
            if cubes.contains(&c2) == false {
                nb_faces += 1;
            }
        }
    }

    nb_faces
}

#[derive(Eq, PartialEq)]
enum Symbol {
    Empty,
    Cube,
    Perimeter
}

fn build_cubes_map(cubes: &HashSet<Coordinate3D<isize>>) -> Grid3D<isize, Symbol> {
    let mut grid: Grid3D<isize, Symbol> = Grid3D::new();

    for c in cubes.iter() {
        grid.add_location(c.clone(), Symbol::Cube);
    }

    let (min_z, max_z): (isize, isize) = (grid.min_z().into(), grid.max_z().into());
    let (min_y, max_y): (isize, isize) = (grid.min_y().into(), grid.max_y().into());
    let (min_x, max_x): (isize, isize) = (grid.min_x().into(), grid.max_x().into());

    // Filling rest of 3d space so all values are defined and adding perimeter for shortest path
    for z in min_z - 1..max_z + 2 {
        for y in min_y - 1..max_y + 2 {
            for x in min_x - 1..max_x + 2 {
                if grid.get_value(&Coordinate3D { x, y, z }) == None {
                    // Non Symbol::Cube location
                    match z < min_z || z > max_z || y < min_y || y > max_y || x < min_x || x > max_x {
                        true => grid.add_location(Coordinate3D { x, y, z }, Symbol::Perimeter),
                        false => grid.add_location(Coordinate3D { x, y, z }, Symbol::Empty)
                    };
                }
            }
        }
    }

    grid
}

fn count_external_faces(cubes: &HashSet<Coordinate3D<isize>>, cubes_map: &Grid3D<isize, Symbol>) -> usize {
    let mut nb_faces = 0;
    let mut adjacent_empty_spaces: HashMap<Coordinate3D<isize>, usize> = HashMap::new();

    // Counting non touching faces
    println!("Mapping non touching faces...");
    for c in cubes.iter() {
        // Two unit cubes are touching in one direction if coordinate are adjacent
        for c2 in get_adjacent_orthogonal_locations(&c).into_iter() {
            if cubes.contains(&c2) == false {
                nb_faces += 1;

                // Keeping track of connected empty spaces to validate later
                match adjacent_empty_spaces.get(&c2) {
                    Some(v) => adjacent_empty_spaces.insert(c2, v + 1),
                    None => adjacent_empty_spaces.insert(c2, 1)
                };
            }
        }
    }

    // Validating connected empty spaces adjacent to non touching faces
    println!("Validating {} empty spaces connected to {} faces...", adjacent_empty_spaces.len(), adjacent_empty_spaces.values().sum::<usize>());
    let goal = PathTarget::Value(Symbol::Perimeter); // Any value on external perimeter boundary
    for (c, nb_touching_faces) in adjacent_empty_spaces.iter() {
        let location_validator: Box<dyn Fn(&Symbol, &Symbol) -> bool> = Box::new(|_: &Symbol, next: &Symbol| -> bool { *next != Symbol::Cube });
        if find_shortest_path(cubes_map, &c, &goal, Some(location_validator)) == None {
            nb_faces -= nb_touching_faces; // Uncounting all touching faces from before
        }
    }

    nb_faces
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
        assert_eq!(64, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(58, solution2(data));
    }
}
