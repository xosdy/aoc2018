use crate::vm::*;
use std::collections::HashSet;

#[aoc(day21, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut vm: Vm = input.parse().unwrap();
    while *vm.ip() != 28 {
        vm.step();
    }

    vm.registers.0[vm.program[28].input1]
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut vm: Vm = input.parse().unwrap();
    let mut numbers = HashSet::new();
    let mut last = 0;

    loop {
        vm.step();

        if *vm.ip() == 28 {
            if numbers.insert(vm.registers.0[vm.program[28].input1]) {
                last = vm.registers.0[vm.program[28].input1];
            } else {
                break;
            }
        }
    }

    last
}
