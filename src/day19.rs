use crate::vm::*;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vm {
    let mut vm: Vm = input.parse().unwrap();
    vm.registers.0.resize(6, 0);
    vm
}

#[aoc(day19, part1)]
pub fn solve_part1(vm: &Vm) -> usize {
    let mut vm = vm.to_owned();
    vm.run();
    vm.registers.0[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
";

    #[test]
    fn part1() {
        let mut vm = input_generator(TEST_INPUT);
        vm.run();
        assert_eq!(vm.registers.0[0], 7);
    }
}
