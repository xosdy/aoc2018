use std::str::FromStr;

pub const OPCODE_COUNT: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registers(pub Vec<usize>);

impl FromStr for Registers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reg: Registers = Default::default();
        reg.0 = s.split(',').map(|s| s.trim().parse().unwrap()).collect();
        Ok(reg)
    }
}

impl Default for Registers {
    fn default() -> Self {
        Registers(vec![0; 6])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub input1: usize,
    pub input2: usize,
    pub output: usize,
}

impl Instruction {
    pub fn new(opcode: Opcode, code: &[usize]) -> Instruction {
        Instruction {
            opcode,
            input1: code[0],
            input2: code[1],
            output: code[2],
        }
    }

    pub fn execute(&self, registers: &mut Registers) {
        use self::Opcode::*;
        match self.opcode {
            Addr => registers.0[self.output] = registers.0[self.input1] + registers.0[self.input2],
            Addi => registers.0[self.output] = registers.0[self.input1] + self.input2,
            Mulr => registers.0[self.output] = registers.0[self.input1] * registers.0[self.input2],
            Muli => registers.0[self.output] = registers.0[self.input1] * self.input2,
            Banr => registers.0[self.output] = registers.0[self.input1] & registers.0[self.input2],
            Bani => registers.0[self.output] = registers.0[self.input1] & self.input2,
            Borr => registers.0[self.output] = registers.0[self.input1] | registers.0[self.input2],
            Bori => registers.0[self.output] = registers.0[self.input1] | self.input2,
            Setr => registers.0[self.output] = registers.0[self.input1],
            Seti => registers.0[self.output] = self.input1,
            Gtir => {
                registers.0[self.output] = if self.input1 > registers.0[self.input2] {
                    1
                } else {
                    0
                }
            }
            Gtri => {
                registers.0[self.output] = if registers.0[self.input1] > self.input2 {
                    1
                } else {
                    0
                }
            }
            Gtrr => {
                registers.0[self.output] = if registers.0[self.input1] > registers.0[self.input2] {
                    1
                } else {
                    0
                }
            }
            Eqir => {
                registers.0[self.output] = if self.input1 == registers.0[self.input2] {
                    1
                } else {
                    0
                }
            }
            Eqri => {
                registers.0[self.output] = if registers.0[self.input1] == self.input2 {
                    1
                } else {
                    0
                }
            }
            Eqrr => {
                registers.0[self.output] = if registers.0[self.input1] == registers.0[self.input2] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        macro_rules! parse {
            ($iter:expr) => {
                $iter.next().and_then(|part| part.parse().ok()).unwrap()
            };
        }

        Ok(Instruction {
            opcode: parse!(iter),
            input1: parse!(iter),
            input2: parse!(iter),
            output: parse!(iter),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Vm {
    pub registers: Registers,
    pub program: Vec<Instruction>,
    ip_index: usize,
}

impl Vm {
    pub fn run(&mut self) {
        while *self.ip() < self.program.len() {
            self.step();
        }
    }

    pub fn ip(&mut self) -> &mut usize {
        &mut self.registers.0[self.ip_index]
    }

    pub fn step(&mut self) {
        let ip = *self.ip();
        self.program[ip].execute(&mut self.registers);
        *self.ip() += 1;
    }
}

impl FromStr for Vm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.lines();
        let ip_index = line_iter
            .next()
            .and_then(|line| line.split_whitespace().next_back())
            .and_then(|ip| ip.parse().ok())
            .unwrap();
        let program = line_iter.map(|line| line.parse().unwrap()).collect();

        Ok(Vm {
            registers: Default::default(),
            ip_index,
            program,
        })
    }
}
