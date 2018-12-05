#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
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

    polymer.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_part1("dabAcCaCBAcCcaDA"), 10);
    }
}
