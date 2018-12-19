use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;
use strum::IntoEnumIterator;

const OPCODE_COUNT: usize = 16;

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

#[derive(Debug, Default, Clone)]
pub struct Sample {
    before: Registers,
    unknown_instruction: UnknownInstruction,
    after: Registers,
}

pub fn guess_opcode(samples: &Vec<Sample>) -> HashMap<usize, Opcode> {
    let mut match_opcodes: Vec<HashSet<Opcode>> = vec![HashSet::new(); OPCODE_COUNT];
    for op_code in 0..OPCODE_COUNT {
        let sets: Vec<HashSet<Opcode>> = samples
            .iter()
            .filter(|s| s.unknown_instruction.0[0] == op_code)
            .map(|s| {
                Opcode::iter()
                    .filter(|op| {
                        let instruction =
                            Instruction::new(op.clone(), &s.unknown_instruction.0[1..4]);
                        let mut registers = s.before.clone();
                        instruction.execute(&mut registers);

                        registers == s.after
                    })
                    .collect::<HashSet<Opcode>>()
            })
            .collect();

        match_opcodes[op_code] = Opcode::iter().collect();
        for set in sets {
            match_opcodes[op_code] = match_opcodes[op_code]
                .intersection(&set)
                .map(|op| op.clone())
                .collect();
        }
    }

    let mut opcode_map = HashMap::new();
    while opcode_map.len() != OPCODE_COUNT {
        let confirm_opcodes: Vec<Opcode> = match_opcodes
            .iter()
            .enumerate()
            .filter(|(_, set)| set.len() == 1)
            .map(|(i, set)| {
                let opcode = set.iter().next().unwrap();
                opcode_map.insert(i, opcode.clone());
                opcode.clone()
            })
            .collect();

        match_opcodes.iter_mut().for_each(|set| {
            confirm_opcodes.iter().for_each(|opcode| {
                set.remove(opcode);
            });
        });
    }

    opcode_map
}

#[derive(Debug, Default, Clone)]
pub struct UnknownInstruction([usize; 4]);

impl FromStr for UnknownInstruction {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instruction: UnknownInstruction = Default::default();
        let code = s
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        if code.len() != 4 {
            return Err(Box::from("Code len != 4"));
        }

        instruction.0.copy_from_slice(&code);
        Ok(instruction)
    }
}

pub type RawProgram = Vec<UnknownInstruction>;

pub struct Input {
    samples: Vec<Sample>,
    raw_program: RawProgram,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut samples = vec![];
    let mut sample: Sample = Default::default();
    let mut sample_end = 0;
    for (i, line) in input.lines().enumerate() {
        if i % 4 == 0 {
            if let Some(begin) = line.find('[') {
                sample.before = line[begin + 1..line.len() - 1].parse().unwrap();
            } else {
                sample_end = i;
                break;
            }
        } else if i % 4 == 1 {
            sample.unknown_instruction = line.parse().unwrap();
        } else if i % 4 == 2 {
            if let Some(begin) = line.find('[') {
                sample.after = line[begin + 1..line.len() - 1].parse().unwrap();
                samples.push(sample.clone());
            } else {
                sample_end = i;
                break;
            }
        }
    }

    let raw_program = input
        .lines()
        .skip(sample_end)
        .filter_map(|s| s.parse().ok())
        .collect();

    Input {
        samples,
        raw_program: raw_program,
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> usize {
    let samples = &input.samples;
    samples
        .iter()
        .filter(|s| {
            Opcode::iter()
                .filter(|op| {
                    let instruction = Instruction::new(op.clone(), &s.unknown_instruction.0[1..4]);
                    let mut registers = s.before.clone();
                    instruction.execute(&mut registers);

                    registers == s.after
                })
                .count()
                >= 3
        })
        .count()
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> usize {
    let opcode_map = guess_opcode(&input.samples);
    let mut registers: Registers = Default::default();
    input
        .raw_program
        .iter()
        .map(|i| Instruction::new(opcode_map.get(&i.0[0]).unwrap().clone(), &i.0[1..4]))
        .for_each(|i| i.execute(&mut registers));

    registers.0[0]
}
