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
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

/// Get direction from string slice
pub fn str_to_direction(d: &str) -> Direction {
    match d {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        _ => panic!("Invalid direction {d}")
    }
}

/// Get all four cardinal adjacent locations to location
pub fn get_adjacent_cardinal_locations<T>(location: &Coordinate<T>) -> Vec<Coordinate<T>>
where
    T: Copy + Add<isize, Output = T> + Sub<isize, Output = T>
{
    vec![
        Coordinate { x: location.x - 1, y: location.y }, // Left
        Coordinate { x: location.x + 1, y: location.y }, // Right
        Coordinate { x: location.x, y: location.y - 1 }, // Up
        Coordinate { x: location.x, y: location.y + 1 }, // Down
    ]
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

    pub fn add_location(&mut self, location: Coordinate<T>, value: V) {
        self.locations.insert(location, value);
    }

    pub fn update_value(&mut self, location: &Coordinate<T>, value: V) {
        if let Some(v) = self.locations.get_mut(location) {
            *v = value;
        }
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
    let mut path_to_explored: Vec<Path<T>> = vec![Path::new(start)];

    let mut path_size = 0;
    while path_to_explored.len() > 0 {
        path_size += 1;
        println!("Path size: {path_size}");

        let current_paths: Vec<Path<T>> = path_to_explored.drain(..).collect();

        for path in current_paths {
            let current_location = path.locations.last().unwrap();
            let current_elevation = map.get_value(&current_location).unwrap();

            let adjacent_locations = get_adjacent_cardinal_locations(&current_location);
            let next_locations: Vec<Coordinate<T>> = adjacent_locations
                .into_iter()
                // Removing locations outside map
                .filter(|l| map.get_value(&l) != None)
                // Removing already visited locations
                .filter(|l| visited_locations.contains(&l) == false)
                .filter(|l| match &location_validator {
                    // Removing invalid locations
                    Some(validator) => (validator)(current_elevation, map.get_value(&l).unwrap()),
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
                path_to_explored.push(next_path);
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
