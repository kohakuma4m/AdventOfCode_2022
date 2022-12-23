use num::Signed;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

/// A 3d Coordinate3D
#[derive(Debug, Copy, Clone)]
pub struct Coordinate3D<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Coordinate3D<T>
where
    T: Copy + Add + Sub + Signed
{
    pub fn manhattan_distance(&self, other: &Coordinate3D<T>) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

// Ordering by (y, x)
impl<T> Ord for Coordinate3D<T>
where
    T: Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        let t1 = (&self.z, &self.y, &self.x);
        let t2 = (&other.z, &other.y, &other.x);

        t1.cmp(&t2)
    }

    fn max(self, other: Self) -> Self {
        let t1 = (&self.z, &self.y, &self.x);
        let t2 = (&other.z, &other.y, &other.x);

        match t1.max(t2) {
            (a, b, c) if a == &self.z && b == &self.y && c == &self.x => self,
            _ => other
        }
    }

    fn min(self, other: Self) -> Self {
        let t1 = (&self.z, &self.y, &self.x);
        let t2 = (&other.z, &other.y, &other.x);

        match t1.min(t2) {
            (a, b, c) if a == &other.z && b == &other.y && c == &other.x => other,
            _ => other
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        let t = (&self.z, &self.y, &self.x);
        let tmin = (&min.z, &min.y, &min.x);
        let tmax = (&max.z, &max.y, &max.x);

        match t.clamp(tmin, tmax) {
            (a, b, c) if a == &min.z && b == &min.y && c == &min.x => min,
            (a, b, c) if a == &min.z && b == &max.y && c == &max.x => max,
            _ => self
        }
    }
}

impl<T> PartialOrd for Coordinate3D<T>
where
    T: Ord
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Equality
impl<T> Eq for Coordinate3D<T> where T: Eq {}

impl<T> PartialEq for Coordinate3D<T>
where
    T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T> Hash for Coordinate3D<T>
where
    T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

/// A 3d direction
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    In,
    Out
}

/// Get all four cardinal adjacent locations to location
pub fn get_adjacent_orthogonal_locations<T>(location: &Coordinate3D<T>) -> Vec<Coordinate3D<T>>
where
    T: Copy + Add<isize, Output = T> + Sub<isize, Output = T>
{
    vec![
        Coordinate3D { x: location.x - 1, y: location.y, z: location.z }, // Left
        Coordinate3D { x: location.x + 1, y: location.y, z: location.z }, // Right
        Coordinate3D { x: location.x, y: location.y - 1, z: location.z }, // Up
        Coordinate3D { x: location.x, y: location.y + 1, z: location.z }, // Down
        Coordinate3D { x: location.x, y: location.y, z: location.z - 1 }, // In
        Coordinate3D { x: location.x, y: location.y, z: location.z + 1 }, // Out
    ]
}

/// A 2d grid implemented as an hash map of locations
pub struct Grid3D<T, V> {
    locations: HashMap<Coordinate3D<T>, V>
}

impl<T, V> Grid3D<T, V>
where
    T: Ord + Copy + Hash + From<isize> + Into<isize>,
    V: Eq
{
    pub fn new() -> Grid3D<T, V> {
        Grid3D { locations: HashMap::new() }
    }

    pub fn from(grid: &Grid3D<T, V>) -> Grid3D<T, V>
    where
        T: Copy,
        V: Copy
    {
        Grid3D { locations: grid.locations.clone() }
    }

    pub fn is_empty(&self) -> bool {
        self.locations.is_empty()
    }

    pub fn size(&self) -> usize {
        self.locations.len()
    }

    pub fn add_location(&mut self, location: Coordinate3D<T>, value: V) -> Option<V> {
        return self.locations.insert(location, value);
    }

    pub fn get_value(&self, location: &Coordinate3D<T>) -> Option<&V> {
        self.locations.get(location)
    }

    pub fn count_values(&self, value: &V) -> usize {
        self.locations.values().filter(|v| *v == value).count()
    }

    pub fn get_locations_with_value(&self, value: &V) -> Vec<Coordinate3D<T>> {
        let (min_x, max_x): (isize, isize) = (self.min_x().into(), self.max_x().into());
        let (min_y, max_y): (isize, isize) = (self.min_y().into(), self.max_y().into());
        let (min_z, max_z): (isize, isize) = (self.min_z().into(), self.max_z().into());

        let mut locations = vec![];
        for z in min_z..max_z + 1 {
            for y in min_y..max_y + 1 {
                for x in min_x..max_x + 1 {
                    let location = Coordinate3D { x: T::from(x), y: T::from(y), z: T::from(z) };
                    match self.locations.get(&location) {
                        Some(v) => {
                            if v == value {
                                locations.push(location);
                            }
                        },
                        None => continue
                    }
                }
            }
        }

        locations
    }

    pub fn get_mapped_locations_with_value(&self, value: &V) -> Vec<Coordinate3D<T>> {
        self.locations.iter().filter(|(_, v)| *v == value).map(|(l, _)| Coordinate3D { x: l.x, y: l.y, z: l.z }).collect()
    }

    pub fn min_x(&self) -> T {
        self.locations.keys().min_by_key(|l| l.x).unwrap().x
    }

    pub fn min_y(&self) -> T {
        self.locations.keys().min_by_key(|l| l.y).unwrap().y
    }

    pub fn min_z(&self) -> T {
        self.locations.keys().min_by_key(|l| l.z).unwrap().z
    }

    pub fn max_x(&self) -> T {
        self.locations.keys().max_by_key(|l| l.x).unwrap().x
    }

    pub fn max_y(&self) -> T {
        self.locations.keys().max_by_key(|l| l.y).unwrap().y
    }

    pub fn max_z(&self) -> T {
        self.locations.keys().max_by_key(|l| l.z).unwrap().z
    }

    pub fn width(&self) -> T {
        let (min_x, max_x): (isize, isize) = (self.min_x().into(), self.max_x().into());

        if max_x < 0 || min_x > 0 {
            T::from(max_x - min_x)
        }
        else {
            T::from(max_x - min_x + 1) // Adding 1 for zero
        }
    }

    pub fn height(&self) -> T {
        let (min_y, max_y): (isize, isize) = (self.min_y().into(), self.max_y().into());

        if max_y < 0 || min_y > 0 {
            T::from(max_y - min_y)
        }
        else {
            T::from(max_y - min_y + 1) // Adding 1 for zero
        }
    }

    pub fn depth(&self) -> T {
        let (min_z, max_z): (isize, isize) = (self.min_z().into(), self.max_z().into());

        if max_z < 0 || min_z > 0 {
            T::from(max_z - min_z)
        }
        else {
            T::from(max_z - min_z + 1) // Adding 1 for zero
        }
    }

    pub fn print(&self, empty_value: &V, map_value_to_char: &dyn Fn(&V) -> &str) {
        let (min_z, max_z): (isize, isize) = (self.min_z().into(), self.max_z().into());

        for z in min_z..max_z + 1 {
            println!("z = {z}");
            self.print_z_layer(T::from(z), empty_value, map_value_to_char);
            println!("");
        }
    }

    pub fn print_z_layer(&self, z: T, empty_value: &V, map_value_to_char: &dyn Fn(&V) -> &str) {
        let (min_x, max_x): (isize, isize) = (self.min_x().into(), self.max_x().into());
        let (min_y, max_y): (isize, isize) = (self.min_y().into(), self.max_y().into());
        let width = self.width().into();
        let separator = (0..width + 2).map(|_| "-").collect::<String>();

        println!("{separator}");
        for y in min_y..max_y + 1 {
            let mut row = String::from("");
            for x in min_x..max_x + 1 {
                row.push_str(map_value_to_char(self.locations.get(&Coordinate3D { x: T::from(x), y: T::from(y), z }).unwrap_or(empty_value)));
            }
            println!("|{}|", row);
        }
        println!("{separator}");
    }
}

/// A 3d path of locations
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Path<T> {
    pub locations: Vec<Coordinate3D<T>>
}

impl<T> Path<T>
where
    T: Copy
{
    pub fn new(location: &Coordinate3D<T>) -> Path<T> {
        Path { locations: vec![location.clone()] }
    }

    pub fn from(path: &Path<T>) -> Path<T> {
        Path { locations: path.locations.clone() }
    }
}

/// A Path target location or value
pub enum PathTarget<T, V> {
    Location(Coordinate3D<T>),
    Value(V)
}

/// Find first shortest path found between start location and path target
pub fn find_shortest_path<T, V>(
    map: &Grid3D<T, V>,
    start: &Coordinate3D<T>,
    goal: &PathTarget<T, V>,
    location_validator: Option<Box<dyn Fn(&V, &V) -> bool>>
) -> Option<Path<T>>
where
    T: Ord + Copy + Hash + From<isize> + Into<isize> + Add<isize, Output = T> + Sub<isize, Output = T>,
    V: Eq
{
    let mut visited_locations: HashSet<Coordinate3D<T>> = HashSet::new();
    let mut paths_to_explored: Vec<Path<T>> = vec![Path::new(start)];

    while paths_to_explored.len() > 0 {
        let current_paths: Vec<Path<T>> = paths_to_explored.drain(..).collect();

        for path in current_paths {
            let current_location = path.locations.last().unwrap();
            let current_value = map.get_value(&current_location).unwrap();

            let adjacent_locations = get_adjacent_orthogonal_locations(&current_location);
            let next_locations: Vec<Coordinate3D<T>> = adjacent_locations
                .into_iter()
                // Removing locations outside map
                .filter(|l| map.get_value(&l) != None)
                // Removing already visited locations
                .filter(|l| visited_locations.contains(&l) == false)
                .filter(|l| match &location_validator {
                    // Removing invalid locations
                    Some(validator) => (validator)(current_value, map.get_value(&l).unwrap()),
                    None => true
                })
                .collect();

            for location in next_locations.into_iter() {
                let mut next_path = Path::from(&path);
                next_path.locations.push(location.clone());

                if match goal {
                    PathTarget::Location(goal_location) => &location == goal_location,
                    PathTarget::Value(goal_value) => map.get_value(&location).unwrap() == goal_value
                } {
                    // Found path to goal
                    return Some(next_path);
                }

                visited_locations.insert(location);
                paths_to_explored.push(next_path);
            }
        }
    }

    None
}
