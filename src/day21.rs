use crate::vm::*;

#[aoc(day21, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut vm: Vm = input.parse().unwrap();
    while *vm.ip() != 28 {
        vm.step();
    }

    vm.registers.0[3]
}
