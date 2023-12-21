use crate::circuit::{read_instructions, ClockCircuit};

pub fn solution1(data: String) -> isize {
    let instructions = read_instructions(data);
    let mut clock = ClockCircuit::new();

    for i in instructions {
        clock.run_instruction(&i);
    }

    let result = [20, 60, 100, 140, 180, 220].into_iter().map(|n| clock.get_signal_strengh(Some(n))).sum();

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> String {
    let instructions = read_instructions(data);
    let mut clock = ClockCircuit::new();
    clock.run(&instructions);

    let screen = ClockCircuitScreen::from(&clock);

    println!("=========================");
    println!("Solution2: ");
    screen.show();
    println!("=========================");

    screen.value()
}

/////////////////////////////////////////////////

enum Pixel {
    Lit,
    Dark
}

fn pixel_to_string(pixel: &Pixel) -> &str {
    match pixel {
        Pixel::Lit => "#",
        Pixel::Dark => "."
    }
}

/// Clock circuit display screen
struct ClockCircuitScreen {
    lines: Vec<String>
}

impl ClockCircuitScreen {
    /// Constructor
    fn from(clock: &ClockCircuit) -> ClockCircuitScreen {
        let mut lines = vec![];
        for j in 0..6 {
            let mut line = String::from("");

            for i in 0..40 {
                // Getting sprite position
                let sprite_position = clock.get_signal(j * 40 + i + 1).unwrap_or(42);

                // Drawing pixel
                let x = i as isize;
                let pixel = match x >= sprite_position - 1 && x <= sprite_position + 1 {
                    true => Pixel::Lit,
                    false => Pixel::Dark
                };
                line.push_str(pixel_to_string(&pixel));
            }

            lines.push(line);
        }

        ClockCircuitScreen { lines }
    }

    /// Screen value
    fn value(&self) -> String {
        self.lines.join("\n")
    }

    /// Print screen
    fn show(&self) {
        let separator = (0..42).map(|_| "-").collect::<String>();

        println!("{separator}");
        for l in &self.lines {
            println!("|{l}|");
        }
        println!("{separator}");
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
        assert_eq!(13140, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();

        let expected = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ]
        .join("\n");

        assert_eq!(expected, solution2(data), "Screen display not matching expected !");
    }
}
