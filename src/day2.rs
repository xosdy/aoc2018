#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let (mut twos, mut threes) = (0, 0);

    input.lines().for_each(|line| {
        let (mut found_two, mut found_three) = (false, false);

        for c in line.bytes() {
            if found_two && found_three {
                break;
            }

            let count = bytecount::count(line.as_bytes(), c);
            if count == 2 && !found_two {
                found_two = true;
                twos += 1;
            }

            if count == 3 && !found_three {
                found_three = true;
                threes += 1;
            }
        }
    });

    twos * threes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(
                r"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"
            ),
            12
        );
    }
}
