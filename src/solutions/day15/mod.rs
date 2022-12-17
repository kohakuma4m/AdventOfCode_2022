use crate::navigation::Coordinate;
use rand::seq::SliceRandom;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    env
};

const SOLUTION1_Y_VALUE: isize = 2000000;
const SOLUTION2_Y_VALUE: isize = 4000000;

const SOLUTION1_TEST_Y_VALUE: &str = "SOLUTION1_TEST_Y_VALUE";
const SOLUTION2_TEST_Y_VALUE: &str = "SOLUTION2_TEST_Y_VALUE";

pub fn solution1(data: String) -> usize {
    let sensors = read_sensors_data(data);

    let y = match env::var(SOLUTION1_TEST_Y_VALUE) {
        Ok(y) => y.parse::<isize>().unwrap(),
        Err(_) => SOLUTION1_Y_VALUE
    };

    let searched_locations = search_row(sensors, y);
    let result = searched_locations.len();

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> isize {
    let sensors = read_sensors_data(data);

    let size = match env::var(SOLUTION2_TEST_Y_VALUE) {
        Ok(y) => y.parse::<isize>().unwrap(),
        Err(_) => SOLUTION2_Y_VALUE
    };
    let boundary = Boundary { x_min: 0, y_min: 0, x_max: size, y_max: size };

    println!("Mapping all searched locations in {:?}...", boundary);
    println!("-------------------------");
    let searched_ranges = map_searched_row_ranges(&sensors, &boundary);

    println!("=========================");
    println!("Searching for single unsearched location...");
    println!("-------------------------");
    let result = match find_hidden_beacon(&searched_ranges, &boundary) {
        Some(location) => {
            println!("-------------------------");
            println!("Location found at {:?}", location);
            location.x * 4000000 + location.y
        },
        None => panic!("No solution found !")
    };

    println!("");
    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

struct Sensor {
    location: Coordinate<isize>,
    beacon_location: Coordinate<isize>,
    beacon_distance: isize
}

fn read_sensors_data(data: String) -> Vec<Sensor> {
    let sensor_beacon_regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

    data.lines()
        .map(|line| {
            let captures = sensor_beacon_regex.captures(line).unwrap();

            let sensor_location = Coordinate { x: captures[1].parse::<isize>().unwrap(), y: captures[2].parse::<isize>().unwrap() };
            let beacon_location = Coordinate { x: captures[3].parse::<isize>().unwrap(), y: captures[4].parse::<isize>().unwrap() };
            let distance = sensor_location.manhattan_distance(&beacon_location);

            Sensor { location: sensor_location, beacon_location: beacon_location, beacon_distance: distance }
        })
        .collect()
}

fn print_sensor(sensor_idx: usize, sensor: &Sensor) {
    println!(
        "sensor #{:<2} ({:7}, {:7}) --> beacon ({:7}, {:7}) at distance |{:7}|",
        sensor_idx + 1,
        sensor.location.x,
        sensor.location.y,
        sensor.beacon_location.x,
        sensor.beacon_location.y,
        sensor.beacon_distance
    );
}

// Mapping all unique searched locations in row
fn search_row(sensors: Vec<Sensor>, y: isize) -> HashSet<Coordinate<isize>> {
    let mut searched_locations: HashSet<Coordinate<isize>> = HashSet::new();

    for (sensor_idx, sensor) in sensors.iter().enumerate() {
        // Sensor signal is a diamond shape with radius = distance
        let dy = (sensor.location.y - y).abs();

        if dy > sensor.beacon_distance {
            continue; // Target row is how of reach
        }

        print_sensor(sensor_idx, &sensor);

        // Sensor signal is covering part of row
        let dx = sensor.beacon_distance - dy;
        let x_min = sensor.location.x - dx;
        let x_max = sensor.location.x + dx;

        for x in x_min..x_max + 1 {
            let location = Coordinate { x, y };
            if location != sensor.beacon_location {
                searched_locations.insert(Coordinate { x, y });
            }
        }
    }

    searched_locations
}

#[derive(Debug)]
struct Range {
    min: isize,
    max: isize
}

#[derive(Debug)]
struct Boundary {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize
}

// Mapping all searched location ranges for all rows inside boundary
fn map_searched_row_ranges(sensors: &Vec<Sensor>, boundary: &Boundary) -> HashMap<isize, Vec<Range>> {
    let mut searched_ranges: HashMap<isize, Vec<Range>> = HashMap::new();

    for (sensor_idx, sensor) in sensors.iter().enumerate() {
        print_sensor(sensor_idx, &sensor);

        // Sensor signal is a diamond shape with radius = distance
        let delta = sensor.beacon_distance;
        for n in 0..delta + 1 {
            let mut x_min = sensor.location.x - (delta - n);
            let mut x_max = sensor.location.x + (delta - n);
            if x_max < boundary.x_min || x_min > boundary.x_max {
                continue; // Outside search range
            }

            // Limiting range
            x_min = max(x_min, 0);
            x_max = min(x_max, boundary.x_max);

            let y_values = match n == 0 {
                true => vec![sensor.location.y],
                false => vec![sensor.location.y - n, sensor.location.y + n]
            };

            for y in y_values.into_iter() {
                if y < boundary.y_min || y > boundary.y_max {
                    continue; // Outside search range
                }

                if let Some(ranges) = searched_ranges.get_mut(&y) {
                    ranges.push(Range { min: x_min, max: x_max });
                }
                else {
                    searched_ranges.insert(y, vec![Range { min: x_min, max: x_max }]);
                };
            }
        }
    }

    // Sorting all ranges
    searched_ranges.values_mut().for_each(|ranges| ranges.sort_by(|r1, r2| r1.min.cmp(&r2.min)));

    searched_ranges
}

fn find_hidden_beacon(searched_ranges: &HashMap<isize, Vec<Range>>, boundary: &Boundary) -> Option<Coordinate<isize>> {
    let mut n = 0;

    // Randomizing rows in case we are lucky...
    let mut rng = rand::thread_rng();
    let mut y_values: Vec<isize> = (0..boundary.y_max + 1).collect();
    y_values.shuffle(&mut rng);

    for y in y_values.into_iter() {
        let ranges = searched_ranges.get(&y).expect("All row should be partially searched!");

        // Searching row sorted searched range intervals to see if there is any gap...
        let mut x = 0;
        for r in ranges.iter() {
            if x < r.min {
                // Found only possible unsearched location
                return Some(Coordinate { x, y });
            }

            if x <= r.max {
                // Skipping all searched values in range...
                x = r.max + 1;
            }
        }

        n += 1;
        if boundary.y_max < 100 || n % (boundary.y_max / 100) == 0 {
            println!("Searching all rows at random ({:4.1}%)...", 100.0 * n as f32 / boundary.y_max as f32);
        }
    }

    None
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

        env::set_var(SOLUTION1_TEST_Y_VALUE, "10");

        assert_eq!(26, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();

        env::set_var(SOLUTION2_TEST_Y_VALUE, "20");

        assert_eq!(56000011, solution2(data));
    }
}
