use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Registers([usize; 4]);

impl FromStr for Registers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reg: Registers = Default::default();
        reg.0.copy_from_slice(
            &s.split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<_>>(),
        );
        Ok(reg)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum OpCode {
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
    op_code: OpCode,
    input1: usize,
    input2: usize,
    output: usize,
}

impl Instruction {
    pub fn new(op_code: OpCode, code: &[usize]) -> Instruction {
        Instruction {
            op_code,
            input1: code[0],
            input2: code[1],
            output: code[2],
        }
    }

    pub fn execute(&self, registers: &mut Registers) {
        use self::OpCode::*;
        match self.op_code {
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

#[derive(Debug, Default, Clone)]
pub struct Sample {
    before: Registers,
    unknown_instruction: [usize; 4],
    after: Registers,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Sample> {
    let mut samples = vec![];
    let mut sample: Sample = Default::default();
    for (i, line) in input.lines().enumerate() {
        if i % 4 == 0 {
            if let Some(begin) = line.find('[') {
                sample.before = line[begin + 1..line.len() - 1].parse().unwrap();
            } else {
                break;
            }
        } else if i % 4 == 1 {
            sample.unknown_instruction.copy_from_slice(
                &line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>(),
            );
        } else if i % 4 == 2 {
            if let Some(begin) = line.find('[') {
                sample.after = line[begin + 1..line.len() - 1].parse().unwrap();
                samples.push(sample.clone());
            } else {
                break;
            }
        }
    }

    samples
}

#[aoc(day16, part1)]
pub fn part1(samples: &Vec<Sample>) -> usize {
    samples
        .iter()
        .filter(|s| {
            OpCode::iter()
                .filter(|op| {
                    let instruction = Instruction::new(op.clone(), &s.unknown_instruction[1..4]);
                    let mut registers = s.before.clone();
                    instruction.execute(&mut registers);

                    registers == s.after
                })
                .count()
                >= 3
        })
        .count()
}
