/// Type of instructions for clock circuit program
#[derive(Debug)]
pub enum InstructionType {
    Noop,
    AddX
}

/// Instruction for clock circuit program
#[derive(Debug)]
pub struct Instruction {
    kind: InstructionType,
    value: Option<isize>
}

/// Method to read instructions for clock circuit program
pub fn read_instructions(data: String) -> Vec<Instruction> {
    data.lines()
        .map(|l| {
            let mut line_iter = l.split_whitespace();
            let instruction_name = line_iter.next().unwrap();

            match instruction_name {
                "noop" => Instruction { kind: InstructionType::Noop, value: None },
                "addx" => {
                    let instruction_value = line_iter.next().unwrap().parse::<isize>().unwrap();
                    Instruction { kind: InstructionType::AddX, value: Some(instruction_value) }
                },
                _ => panic!("Invalid instruction: {l}")
            }
        })
        .collect()
}

/// Clock circuit program
pub struct ClockCircuit {
    cycle: usize,
    register: isize,
    signal: Vec<isize>
}

impl ClockCircuit {
    /// Constructor
    pub fn new() -> ClockCircuit {
        // Starting signal with one value, to simulated change of value at the end of cycle
        ClockCircuit { cycle: 0, register: 1, signal: vec![1] }
    }

    /// Run multiple instructions
    pub fn run(&mut self, instructions: &Vec<Instruction>) {
        for i in instructions {
            self.run_instruction(&i);
        }
    }

    /// Run a single instruction
    pub fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction.kind {
            InstructionType::Noop => {
                self.signal.push(self.register);
                self.cycle += 1;
            },
            InstructionType::AddX => {
                self.signal.push(self.register);
                self.cycle += 2;

                self.register += instruction.value.unwrap();
                self.signal.push(self.register);
            }
        }
    }

    /// Get signal strength at the beginning of cycle or current signal strenght
    pub fn get_signal_strengh(&self, cycle_number: Option<usize>) -> isize {
        match cycle_number {
            Some(n) => n as isize * self.signal[n - 1],
            None => self.cycle as isize * self.register
        }
    }

    /// Get signal value at the beginning of cycle
    pub fn get_signal(&self, cycle_number: usize) -> Option<isize> {
        match cycle_number > 0 && cycle_number <= self.cycle {
            true => Some(self.signal[cycle_number - 1]),
            false => None
        }
    }
}
