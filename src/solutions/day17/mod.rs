use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::navigation::{Coordinate, Grid};

pub fn solution1(data: String) -> usize {
    let jet_patterns = read_jet_patterns(data);
    let mut chamber = Chamber::new(jet_patterns);
    chamber.simulate_boulders(2022);

    #[cfg(test)]
    {
        chamber.grid.print(&Symbol::Empty, &symbol_to_char);
    }

    let result = chamber.rock_tower_height;

    println!("=========================");
    println!("Solution1: {}", result);
    println!("=========================");

    result
}

pub fn solution2(data: String) -> usize {
    let jet_patterns = read_jet_patterns(data);
    let mut chamber = Chamber::new(jet_patterns);
    chamber.simulate_boulders(1000000000000);

    // TODO: find repeating pattern to skip ahead (1 trillion is too much...)

    let result = chamber.rock_tower_height;

    println!("=========================");
    println!("Solution2: {}", result);
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug)]
enum JetPattern {
    Left,
    Right
}

fn read_jet_patterns(data: String) -> Vec<JetPattern> {
    data.chars().map(|c| match c {
        '<' => JetPattern::Left,
        '>' => JetPattern::Right,
        _ => panic!("Invalid jet pattern")
    }).collect()
}

const CHAMBER_WIDTH: isize = 7;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Symbol {
    Rock,
    Empty
}

fn symbol_to_char(symbol: &Symbol) -> &str {
    match symbol {
        Symbol::Rock => "#",
        Symbol::Empty => "."
    }
}

const NB_BOULDER_SHAPES: u8 = 5;

#[derive(Debug, Copy, Clone, FromPrimitive)]
enum BoulderShape {
    HBar = 0,
    Plus = 1,
    MirrorL = 2,
    VBar = 3,
    Block = 4
}

#[derive(Debug)]
struct Boulder {
    shape: BoulderShape,
    bottom_left_corner: Coordinate<isize>
}

impl Boulder {
    fn get_coordinates(&self) -> Vec<Coordinate<isize>> {
        match self.shape {
            BoulderShape::HBar => {
                // .####.
                vec![
                    Coordinate { x: self.bottom_left_corner.x, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x + 1, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x + 2, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x + 3, y: self.bottom_left_corner.y }
                ]
            },
            BoulderShape::Plus => {
                // .#.
                // ###
                // .#.
                vec![
                    Coordinate { x: self.bottom_left_corner.x + 1, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x, y: self.bottom_left_corner.y - 1 },
                    Coordinate { x: self.bottom_left_corner.x + 1, y: self.bottom_left_corner.y - 1 },
                    Coordinate { x: self.bottom_left_corner.x + 1, y: self.bottom_left_corner.y - 2 },
                    Coordinate { x: self.bottom_left_corner.x + 2, y: self.bottom_left_corner.y - 1 }
                ]
            },
            BoulderShape::MirrorL => {
                // ..#
                // ..#
                // ###
                vec![
                    Coordinate { x: self.bottom_left_corner.x, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x + 1, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x + 2, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x + 2, y: self.bottom_left_corner.y - 1 },
                    Coordinate { x: self.bottom_left_corner.x + 2, y: self.bottom_left_corner.y - 2 }
                ]
            },
            BoulderShape::VBar => {
                // #
                // #
                // #
                // #
                vec![
                    Coordinate { x: self.bottom_left_corner.x, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x, y: self.bottom_left_corner.y - 1 },
                    Coordinate { x: self.bottom_left_corner.x, y: self.bottom_left_corner.y - 2 },
                    Coordinate { x: self.bottom_left_corner.x, y: self.bottom_left_corner.y - 3 }
                ]
            },
            BoulderShape::Block => {
                // ##
                // ##
                vec![
                    Coordinate { x: self.bottom_left_corner.x, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x + 1, y: self.bottom_left_corner.y },
                    Coordinate { x: self.bottom_left_corner.x, y: self.bottom_left_corner.y - 1 },
                    Coordinate { x: self.bottom_left_corner.x + 1, y: self.bottom_left_corner.y - 1 }
                ]
            }
        }
    }
}

struct Chamber {
    jet_patterns: Vec<JetPattern>,
    grid: Grid<isize, Symbol>,
    next_boulder_value: u8,
    rock_tower_height: usize
}

impl Chamber {
    fn new(mut jet_patterns: Vec<JetPattern>) -> Chamber {
        Chamber { jet_patterns, grid: Grid::new(), next_boulder_value: 0, rock_tower_height: 0 }
    }

    fn spawn_next_boulder(&mut self) -> Boulder {
        let shape = FromPrimitive::from_u8(self.next_boulder_value).expect("Valid boulder value");
        self.next_boulder_value = (self.next_boulder_value + 1) % NB_BOULDER_SHAPES;

        let bottom_left_corner = match self.grid.is_empty() {
            true => Coordinate { x: 2 , y: - 3 as isize },
            false => Coordinate { x: 2 , y: - ((self.rock_tower_height + 3) as isize) } // TODO return 0 for height/width when empty
        };

        let boulder = Boulder { shape, bottom_left_corner };

        // Adding space for new boulder to fit in view
        let y_min = boulder.get_coordinates().iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        for y in y_min..boulder.bottom_left_corner.y + 1 + 3 {
            for x in 0..CHAMBER_WIDTH {
                self.grid.add_location(Coordinate { x, y }, Symbol::Empty);
            }
        }

        boulder
    }

    fn is_boulder_valid(&self, boulder: Boulder) -> bool {
        for c in boulder.get_coordinates() {
            match self.grid.get_value(&Coordinate { x: c.x, y: c.y }) {
                None | Some(Symbol::Rock) => return false,
                Some(Symbol::Empty) => continue
            }
        }

        true
    }

    fn apply_jet_to_boulder(&self, jet: &JetPattern, boulder: &mut Boulder) {
        let x = match jet {
            JetPattern::Left => boulder.bottom_left_corner.x - 1,
            JetPattern::Right => boulder.bottom_left_corner.x + 1
        };

        if self.is_boulder_valid(Boulder { shape: boulder.shape, bottom_left_corner: Coordinate { x, y: boulder.bottom_left_corner.y } }) {
            boulder.bottom_left_corner.x = x;
        }
    }

    fn move_boulder_down(&self, boulder: &mut Boulder) -> bool {
        let y = boulder.bottom_left_corner.y + 1;

        if self.is_boulder_valid(Boulder { shape: boulder.shape, bottom_left_corner: Coordinate { x: boulder.bottom_left_corner.x, y } }) {
            boulder.bottom_left_corner.y = y;
            return true;
        }

        false
    }

    fn simulate_boulders(&mut self, nb_boulders: usize) {
        let mut j = 0;
        let mut n = 0;

        while n < nb_boulders {
            // Next boulder
            let mut boulder = self.spawn_next_boulder();

            loop {
                // Repeating jet pattern
                let jet = &self.jet_patterns[j % self.jet_patterns.len()];
                self.apply_jet_to_boulder(&jet, &mut boulder);
                j += 1;

                // Falling boulder
                if self.move_boulder_down(&mut boulder) == false {
                    break;
                };
            }

            // Mapping boulder
            for c in boulder.get_coordinates() {
                self.grid.add_location(Coordinate { x: c.x, y: c.y }, Symbol::Rock);

                if 1 + c.y.abs() as usize > self.rock_tower_height {
                    // Updating rock tower height
                    self.rock_tower_height = 1 + c.y.abs() as usize;
                }
            }

            n += 1;

            // Cleaning up old coordinate to keep memory usage low...
            self.grid.keep_only_matching_locations(&|c: &Coordinate<isize>, _| -> bool { self.rock_tower_height as isize - c.y.abs() < 100 });

            if n % 1000000 == 0 {
                println!("Boulder #{n} --> height = {} --> size {}", self.rock_tower_height, self.grid.size());
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
        assert_eq!(3068, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(1514285714288, solution2(data));
    }
}
