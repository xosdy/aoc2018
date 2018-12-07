use std::collections::{BTreeMap, HashSet};

pub type StepTree = BTreeMap<String, Vec<String>>;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Box<StepTree> {
    let mut step_tree: StepTree = BTreeMap::new();

    input.lines().for_each(|line| {
        // Step example
        // Step C must be finished before step A can begin.
        let parts: Vec<_> = line.split(' ').collect();
        step_tree.entry(parts[1].to_owned()).or_default();
        step_tree
            .entry(parts[7].to_owned())
            .and_modify(|e| e.push(parts[1].to_owned()))
            .or_insert(vec![parts[1].to_owned()]);
    });

    Box::new(step_tree)
}

#[aoc(day7, part1)]
pub fn solve_part1(step_tree: &StepTree) -> String {
    let mut available_steps = HashSet::<String>::new();

    let mut sequence = String::new();
    while available_steps.len() != step_tree.len() {
        for (name, dependencies) in step_tree.iter() {
            if !available_steps.contains(name)
                && (dependencies.is_empty()
                    || dependencies.iter().all(|dep| available_steps.contains(dep)))
            {
                available_steps.insert(name.clone());
                sequence.push_str(name);
                break;
            }
        }
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator(
                r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            )),
            "CABDFE"
        );
    }
}
