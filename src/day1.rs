use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut records = HashSet::new();
    records.insert(0);
    let mut current_frequency = 0;

    for change in input.iter().cycle() {
        current_frequency += change;

        if !records.insert(current_frequency) {
            break;
        }
    }

    current_frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator(
                r"+1
+1
+1"
            )),
            3,
        );

        assert_eq!(
            solve_part1(&input_generator(
                r"+1
+1
-2"
            )),
            0
        );

        assert_eq!(
            solve_part1(&input_generator(
                r"-1
-2
-3"
            )),
            -6
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(&input_generator(
                r"+1
-1"
            )),
            0
        );

        assert_eq!(
            solve_part2(&input_generator(
                r"+3
+3
+4
-2
-4"
            )),
            10
        );

        assert_eq!(
            solve_part2(&input_generator(
                r"-6
+3
+8
+5
-6"
            )),
            5
        );

        assert_eq!(
            solve_part2(&input_generator(
                r"+7
+7
-2
-7
-4"
            )),
            14
        );
    }
}
