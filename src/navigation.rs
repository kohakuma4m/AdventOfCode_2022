use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

pub fn get_direction(d: &str) -> Direction {
    match d {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        _ => panic!("Invalid direction {d}")
    }
}

/// A 2d grid as an hash map of locations
pub struct Grid<T, V> {
    locations: HashMap<Coordinate<T>, V>
}

impl<C, V> Grid<C, V>
where
    C: Ord + Copy + From<isize> + Into<isize>,
    V: Eq + PartialEq + Hash,
    Coordinate<C>: Eq + PartialEq + Hash
{
    pub fn new() -> Grid<C, V> {
        Grid { locations: HashMap::new() }
    }

    pub fn add_location(&mut self, location: Coordinate<C>, value: V) {
        self.locations.insert(location, value);
    }

    pub fn count_values(&self, value: &V) -> usize {
        self.locations.values().filter(|v| *v == value).count()
    }

    pub fn min_x(&self) -> C {
        self.locations.keys().min_by_key(|l| l.x).unwrap().x
    }

    pub fn min_y(&self) -> C {
        self.locations.keys().min_by_key(|l| l.y).unwrap().y
    }

    pub fn max_x(&self) -> C {
        self.locations.keys().max_by_key(|l| l.x).unwrap().x
    }

    pub fn max_y(&self) -> C {
        self.locations.keys().max_by_key(|l| l.y).unwrap().y
    }

    pub fn print(&self, empty_value: &V, map_value_to_char: &dyn Fn(&V) -> &str) {
        let (min_x, max_x): (isize, isize) = (self.min_x().into(), self.max_x().into());
        let (min_y, max_y): (isize, isize) = (self.min_y().into(), self.max_y().into());
        let width = max_x - min_x + 1;
        let separator = (0..width + 2).map(|_| "-").collect::<String>();

        println!("{separator}");
        for y in min_y..max_y + 1 {
            let mut row = String::from("");
            for x in min_x..max_x + 1 {
                row.push_str(map_value_to_char(self.locations.get(&Coordinate { x: C::from(x), y: C::from(y) }).unwrap_or(empty_value)));
            }
            println!("|{}|", row);
        }
        println!("{separator}");
    }
}
