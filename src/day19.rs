use crate::vm::*;

pub fn factorization(num: usize) -> Vec<usize> {
    let mut factors = vec![];
    for i in 1..=num {
        if num % i == 0 {
            factors.push(i);
        }
    }

    factors
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vm {
    let mut vm: Vm = input.parse().unwrap();
    vm
}

#[aoc(day19, part1)]
pub fn solve_part1(vm: &Vm) -> usize {
    let mut vm = vm.to_owned();
    vm.run();
    vm.registers.0[0]
}

#[aoc(day19, part2)]
pub fn solve_part2(vm: &Vm) -> usize {
    // Base on my input, running instruction #0, #17-35, #1-2 will initialize registers like this:
    //                           ip
    // [0, 1, 10551376, 10550400, 3, 1]
    //
    // Instruction #3-15 equivalent to:
    // for r1 in 1..=r2 {
    //     for r5 in 1..=r2 {
    //         if r1 * r5 == r2 {
    //             r0 += r1;
    //         }
    //     }
    // }
    // That's finding the sum of factors of r2.

    let mut vm = vm.to_owned();
    vm.registers.0[0] = 1;
    loop {
        vm.step();

        if *vm.ip() == 3 {
            break;
        }
    }

    // Use the largest register to factorization
    let &num = vm.registers.0.iter().max().unwrap();
    factorization(num).iter().sum()
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
