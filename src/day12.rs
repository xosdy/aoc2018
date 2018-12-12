use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone)]
pub struct Rule {
    states: BTreeSet<i32>,
    spread_rules: HashSet<String>,
}

impl Rule {
    pub fn next_generation(&mut self) {
        let mut new_states = BTreeSet::new();
        let range =
            *self.states.iter().next().unwrap() - 2..=*self.states.iter().next_back().unwrap() + 2;
        for i in range {
            let pat = (i - 2..=i + 2)
                .map(|i| if self.states.contains(&i) { '#' } else { '.' })
                .collect::<String>();
            if self.spread_rules.contains(&pat) {
                new_states.insert(i);
            }
        }

        self.states = new_states;
    }

    pub fn sum(&self) -> i32 {
        self.states.iter().sum()
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Rule {
    let lines: Vec<_> = input.lines().collect();

    let states = lines[0][lines[0].find(':').unwrap() + 2..]
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i as i32)
        .collect();

    let spread_rules: HashSet<_> = lines[2..]
        .iter()
        .filter(|line| line.ends_with('#'))
        .map(|line| line[0..5].to_owned())
        .collect();

    Rule {
        states,
        spread_rules,
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(rule: &Rule) -> i32 {
    let mut rule = rule.clone();
    for _ in 0..20 {
        rule.next_generation();
    }

    rule.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator(
                r"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
            )),
            325
        );
    }
}
