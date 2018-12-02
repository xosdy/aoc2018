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

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> String {
    for (line_index, line1) in input.lines().enumerate() {
        for line2 in input.lines().skip(line_index + 1) {
            let mut diff_count = 0;
            let mut last_diff_index = 0;
            for (i, (c1, c2)) in line1.chars().zip(line2.chars()).enumerate() {
                if c1 != c2 {
                    diff_count += 1;
                    last_diff_index = i;
                }
            }

            if diff_count == 1 {
                let mut id = line1.to_owned();
                id.remove(last_diff_index);
                return id;
            }
        }
    }

    unreachable!();
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

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(
                r"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
            ),
            "fgij"
        );
    }
}
