use std::collections::{BTreeMap, HashSet};

pub type StepTree = BTreeMap<char, Vec<char>>;

#[derive(Debug)]
pub enum Worker {
    Idle,
    Busy(Work),
}

#[derive(Debug)]
pub struct Work {
    handling_step: char,
    remaining_seconds: u32,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Box<StepTree> {
    let mut step_tree: StepTree = BTreeMap::new();

    input.lines().for_each(|line| {
        // Step example
        // Step C must be finished before step A can begin.
        let parts: Vec<_> = line.split(' ').collect();
        step_tree
            .entry(parts[1].chars().next().unwrap())
            .or_default();
        step_tree
            .entry(parts[7].chars().next().unwrap())
            .and_modify(|e| e.push(parts[1].chars().next().unwrap()))
            .or_insert(vec![parts[1].chars().next().unwrap()]);
    });

    Box::new(step_tree)
}

#[aoc(day7, part1)]
pub fn solve_part1(step_tree: &StepTree) -> String {
    let mut finished_steps = HashSet::<char>::new();

    let mut sequence = String::new();
    while finished_steps.len() != step_tree.len() {
        for (name, dependencies) in step_tree.iter() {
            if !finished_steps.contains(name)
                && (dependencies.is_empty()
                    || dependencies.iter().all(|dep| finished_steps.contains(dep)))
            {
                finished_steps.insert(*name);
                sequence.push(*name);
                break;
            }
        }
    }

    sequence
}

#[aoc(day7, part2)]
pub fn solve_part2(step_tree: &StepTree) -> u32 {
    let mut workers = Vec::new();
    workers.resize_with(5, || Worker::Idle);
    let mut finished_steps = HashSet::<char>::new();
    let mut working_steps = HashSet::<char>::new();
    let mut total_seconds = 0;

    while finished_steps.len() != step_tree.len() {
        // Load step to idle workers
        workers
            .iter_mut()
            .filter(|w| match w {
                Worker::Idle => true,
                Worker::Busy(_) => false,
            })
            .for_each(|w| {
                // Find next step
                for (name, dependencies) in step_tree.iter() {
                    if !finished_steps.contains(name)
                        && !working_steps.contains(name)
                        && (dependencies.is_empty()
                            || dependencies.iter().all(|dep| finished_steps.contains(dep)))
                    {
                        working_steps.insert(*name);
                        *w = Worker::Busy(Work {
                            handling_step: *name,
                            remaining_seconds: *name as u32 - 'A' as u32 + 61,
                        });
                        break;
                    }
                }
            });

        let min_seconds = workers
            .iter()
            .filter_map(|w| match w {
                Worker::Idle => None,
                Worker::Busy(x) => Some(x),
            })
            .min_by_key(|w| w.remaining_seconds)
            .unwrap()
            .remaining_seconds;
        total_seconds += min_seconds;

        for worker in workers.iter_mut() {
            if let Worker::Busy(w) = worker {
                w.remaining_seconds -= min_seconds;

                if w.remaining_seconds == 0 {
                    finished_steps.insert(w.handling_step);
                    working_steps.remove(&w.handling_step);
                    *worker = Worker::Idle;
                }
            }
        }
    }

    total_seconds
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

    #[test]
    fn part2() {
        // With 5 worker and 60+ second
        assert_eq!(
            solve_part2(&input_generator(
                r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            )),
            253
        );
    }
}
