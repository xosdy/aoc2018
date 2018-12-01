#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    input.iter().sum()
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
}
