use std::str::FromStr;

pub const OPCODE_COUNT: usize = 16;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Registers(pub Vec<usize>);

impl FromStr for Registers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reg: Registers = Default::default();
        reg.0 = s.split(',').map(|s| s.trim().parse().unwrap()).collect();
        Ok(reg)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
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

pub struct Instruction {
    opcode: Opcode,
    input1: usize,
    input2: usize,
    output: usize,
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
