use num::Signed;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

/// A 2d Coordinate
#[derive(Debug, Copy, Clone)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T
}

impl<T> Coordinate<T>
where
    T: Copy + Add + Sub + Signed
{
    pub fn manhattan_distance(&self, other: &Coordinate<T>) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// Ordering by (y, x)
impl<T> Ord for Coordinate<T>
where
    T: Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        let t1 = (&self.y, &self.x);
        let t2 = (&other.y, &other.x);

        t1.cmp(&t2)
    }

    fn max(self, other: Self) -> Self {
        let t1 = (&self.y, &self.x);
        let t2 = (&other.y, &other.x);

        match t1.max(t2) {
            (a, b) if a == &self.y && b == &self.x => self,
            _ => other
        }
    }

    fn min(self, other: Self) -> Self {
        let t1 = (&self.y, &self.x);
        let t2 = (&other.y, &other.x);

        match t1.min(t2) {
            (a, b) if a == &other.y && b == &other.x => other,
            _ => other
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        let t = (&self.y, &self.x);
        let tmin = (&min.y, &min.x);
        let tmax = (&max.y, &max.x);

        match t.clamp(tmin, tmax) {
            (a, b) if a == &min.y && b == &min.x => min,
            (a, b) if a == &max.y && b == &max.x => max,
            _ => self
        }
    }
}

impl<T> PartialOrd for Coordinate<T>
where
    T: Ord
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Equality
impl<T> Eq for Coordinate<T> where T: Eq {}

impl<T> PartialEq for Coordinate<T>
where
    T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Hash for Coordinate<T>
where
    T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

/// A 2d direction
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

/// A 2d cardinal direction
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum CardinalDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

/// Get oposite direction
pub fn get_opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right
    }
}

/// Get all four cardinal adjacent locations
pub fn get_adjacent_orthogonal_locations<T>(location: &Coordinate<T>) -> Vec<Coordinate<T>>
where
    T: Copy + Add<isize, Output = T> + Sub<isize, Output = T>
{
    vec![
        Coordinate { x: location.x, y: location.y - 1 }, // Up
        Coordinate { x: location.x + 1, y: location.y }, // Right
        Coordinate { x: location.x, y: location.y + 1 }, // Down
        Coordinate { x: location.x - 1, y: location.y }, // Left
    ]
}

/// Get all four diagonal adjacent locations
pub fn get_adjacent_diagonal_locations<T>(location: &Coordinate<T>) -> Vec<Coordinate<T>>
where
    T: Copy + Add<isize, Output = T> + Sub<isize, Output = T>
{
    vec![
        Coordinate { x: location.x + 1, y: location.y - 1 }, // Top-Right
        Coordinate { x: location.x + 1, y: location.y + 1 }, // Down-Right
        Coordinate { x: location.x - 1, y: location.y + 1 }, // Down-Left
        Coordinate { x: location.x - 1, y: location.y - 1 }, // Top-Left
    ]
}

/// Get adjacent location in direction
pub fn get_adjacent_locations_in_direction<T>(location: &Coordinate<T>, direction: &Direction) -> Coordinate<T>
where
    T: Copy + Add<isize, Output = T> + Sub<isize, Output = T>
{
    match direction {
        Direction::Up => Coordinate { x: location.x, y: location.y - 1 },
        Direction::Right => Coordinate { x: location.x + 1, y: location.y },
        Direction::Down => Coordinate { x: location.x, y: location.y + 1 },
        Direction::Left => Coordinate { x: location.x - 1, y: location.y }
    }
}

/// Get adjacent location in cardinal direction
pub fn get_adjacent_locations_in_cardinal_direction<T>(location: &Coordinate<T>, direction: &CardinalDirection) -> Coordinate<T>
where
    T: Copy + Add<isize, Output = T> + Sub<isize, Output = T>
{
    match direction {
        CardinalDirection::North => Coordinate { x: location.x, y: location.y - 1 },
        CardinalDirection::NorthEast => Coordinate { x: location.x + 1, y: location.y - 1 },
        CardinalDirection::East => Coordinate { x: location.x + 1, y: location.y },
        CardinalDirection::SouthEast => Coordinate { x: location.x + 1, y: location.y + 1 },
        CardinalDirection::South => Coordinate { x: location.x, y: location.y + 1 },
        CardinalDirection::SouthWest => Coordinate { x: location.x - 1, y: location.y + 1 },
        CardinalDirection::West => Coordinate { x: location.x - 1, y: location.y },
        CardinalDirection::NorthWest => Coordinate { x: location.x - 1, y: location.y - 1 }
    }
}

/// Get direction from location to location
pub fn get_direction_from_adjacent_locations<T>(start: &Coordinate<T>, end: &Coordinate<T>) -> Direction
where
    T: Ord + Eq
{
    if start.y == end.y {
        // Horizontal line
        if start.x < end.x {
            Direction::Right
        }
        else {
            Direction::Left
        }
    }
    else {
        // Vertical line
        if start.y < end.y {
            Direction::Up
        }
        else {
            Direction::Down
        }
    }
}

/// A 2d rotation
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Rotation {
    Clockwise,
    CounterClockwise
}

/// Get new direction after rotation from current direction
pub fn get_direction_after_rotation(direction: &Direction, rotation: &Rotation) -> Direction {
    match rotation {
        Rotation::Clockwise => match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        },
        Rotation::CounterClockwise => match direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down
        }
    }
}

/// A 2d grid implemented as an hash map of locations
pub struct Grid<T, V> {
    locations: HashMap<Coordinate<T>, V>
}

impl<T, V> Grid<T, V>
where
    T: Ord + Copy + Hash + From<isize> + Into<isize>,
    V: Eq
{
    pub fn new() -> Grid<T, V> {
        Grid { locations: HashMap::new() }
    }

    pub fn from(grid: &Grid<T, V>) -> Grid<T, V>
    where
        T: Copy,
        V: Copy
    {
        Grid { locations: grid.locations.clone() }
    }

    pub fn is_empty(&self) -> bool {
        self.locations.is_empty()
    }

    pub fn size(&self) -> usize {
        self.locations.len()
    }

    pub fn add_location(&mut self, location: Coordinate<T>, value: V) -> Option<V> {
        return self.locations.insert(location, value);
    }

    pub fn get_value(&self, location: &Coordinate<T>) -> Option<&V> {
        self.locations.get(location)
    }

    pub fn count_values(&self, value: &V) -> usize {
        self.locations.values().filter(|v| *v == value).count()
    }

    pub fn get_locations_with_value(&self, value: &V) -> Vec<Coordinate<T>> {
        let (min_x, max_x): (isize, isize) = (self.min_x().into(), self.max_x().into());
        let (min_y, max_y): (isize, isize) = (self.min_y().into(), self.max_y().into());

        let mut locations = vec![];
        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                let location = Coordinate { x: T::from(x), y: T::from(y) };
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

        locations
    }

    pub fn get_locations_with_values(&self, values: Vec<&V>) -> Vec<Coordinate<T>> {
        let (min_x, max_x): (isize, isize) = (self.min_x().into(), self.max_x().into());
        let (min_y, max_y): (isize, isize) = (self.min_y().into(), self.max_y().into());

        let mut locations = vec![];
        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                let location = Coordinate { x: T::from(x), y: T::from(y) };
                match self.locations.get(&location) {
                    Some(v) => {
                        if values.contains(&v) {
                            locations.push(location);
                        }
                    },
                    None => continue
                }
            }
        }

        locations
    }

    pub fn get_mapped_locations_with_value(&self, value: &V) -> Vec<Coordinate<T>> {
        self.locations.iter().filter(|(_, v)| *v == value).map(|(l, _)| Coordinate { x: l.x, y: l.y }).collect()
    }

    pub fn keep_only_matching_locations(&mut self, f: &dyn Fn(&Coordinate<T>, &mut V) -> bool) {
        self.locations.retain(f);
    }

    pub fn min_x(&self) -> T {
        self.locations.keys().min_by_key(|l| l.x).unwrap().x
    }

    pub fn min_y(&self) -> T {
        self.locations.keys().min_by_key(|l| l.y).unwrap().y
    }

    pub fn max_x(&self) -> T {
        self.locations.keys().max_by_key(|l| l.x).unwrap().x
    }

    pub fn max_y(&self) -> T {
        self.locations.keys().max_by_key(|l| l.y).unwrap().y
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

    pub fn print(&self, empty_value: &V, map_value_to_char: &dyn Fn(&V) -> &str) {
        let (min_x, max_x): (isize, isize) = (self.min_x().into(), self.max_x().into());
        let (min_y, max_y): (isize, isize) = (self.min_y().into(), self.max_y().into());
        let width = self.width().into();
        let separator = (0..width + 2).map(|_| "-").collect::<String>();

        println!("{separator}");
        for y in min_y..max_y + 1 {
            let mut row = String::from("");
            for x in min_x..max_x + 1 {
                row.push_str(map_value_to_char(self.locations.get(&Coordinate { x: T::from(x), y: T::from(y) }).unwrap_or(empty_value)));
            }
            println!("|{}|", row);
        }
        println!("{separator}");
    }
}

/// A 2d path of locations
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Path<T> {
    pub locations: Vec<Coordinate<T>>
}

impl<T> Path<T>
where
    T: Copy
{
    pub fn new(location: &Coordinate<T>) -> Path<T> {
        Path { locations: vec![location.clone()] }
    }

    pub fn from(path: &Path<T>) -> Path<T> {
        Path { locations: path.locations.clone() }
    }
}

/// A Path target location or value
pub enum PathTarget<T, V> {
    Location(Coordinate<T>),
    Value(V)
}

/// Find first shortest path found between start location and path target
pub fn find_shortest_path<T, V>(
    map: &Grid<T, V>,
    start: &Coordinate<T>,
    goal: &PathTarget<T, V>,
    location_validator: Option<Box<dyn Fn(&V, &V) -> bool>>
) -> Option<Path<T>>
where
    T: Ord + Copy + Hash + From<isize> + Into<isize> + Add<isize, Output = T> + Sub<isize, Output = T>,
    V: Eq
{
    let mut visited_locations: HashSet<Coordinate<T>> = HashSet::new();
    let mut paths_to_explored: Vec<Path<T>> = vec![Path::new(start)];

    let mut path_size = 0;
    while paths_to_explored.len() > 0 {
        path_size += 1;
        println!("Path size: {path_size}");

        let current_paths: Vec<Path<T>> = paths_to_explored.drain(..).collect();

        for path in current_paths {
            let current_location = path.locations.last().unwrap();
            let current_value = map.get_value(&current_location).unwrap();

            let adjacent_locations = get_adjacent_orthogonal_locations(&current_location);
            let next_locations: Vec<Coordinate<T>> = adjacent_locations
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

/// Print a path on top of grid map first shortest path found between start location and path target
pub fn print_path<T, V>(
    path: &Path<T>,
    map: &Grid<T, V>,
    start_value: V,
    end_value: V,
    map_direction_to_value: &dyn Fn(&Direction) -> V,
    empty_value: &V,
    map_value_to_char: &dyn Fn(&V) -> &str
) where
    T: Ord + Copy + Hash + From<isize> + Into<isize>,
    V: Copy + Eq
{
    // New grid to draw on
    let mut new_map = Grid::from(map);

    let path_length = path.locations.len();

    // Start
    if path_length > 0 {
        new_map.add_location(path.locations[0].clone(), start_value);
    }

    // Middle
    for i in 1..path_length {
        let direction = get_direction_from_adjacent_locations(&path.locations[i - 1], &path.locations[i]);
        new_map.add_location(path.locations[0].clone(), map_direction_to_value(&direction));
    }

    // End
    if path_length > 2 {
        new_map.add_location(path.locations.last().unwrap().clone(), end_value);
    }

    new_map.print(empty_value, map_value_to_char);
}
