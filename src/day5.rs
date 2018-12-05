pub fn collapse(input: &str) -> String {
    let mut polymer = String::new();
    for c in input.trim().chars() {
        match polymer.chars().last() {
            None => polymer.push(c),
            Some(last_c) => {
                if c != last_c && c.to_ascii_lowercase() == last_c.to_ascii_lowercase() {
                    polymer.pop();
                } else {
                    polymer.push(c);
                }
            }
        }
    }

    polymer
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    collapse(input).len()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    (b'a'..=b'z')
        .map(|c| {
            let c = c as char;
            let polymer = input
                .to_owned()
                .replace(c, "")
                .replace(c.to_ascii_uppercase(), "");
            collapse(&polymer).len()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_part1("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2("dabAcCaCBAcCcaDA"), 4);
    }
}
