#[derive(Debug, Default)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    pub fn from_flat(iter: &mut impl Iterator<Item = usize>) -> Node {
        let mut node: Node = Default::default();
        let child_count = iter.next().unwrap();
        let metadata_count = iter.next().unwrap();

        for _ in 0..child_count {
            node.children.push(Node::from_flat(iter));
        }

        node.metadata.extend(iter.take(metadata_count));
        node
    }

    pub fn sum_metadata(&self) -> usize {
        self.children
            .iter()
            .fold(self.metadata.iter().sum(), |acc, x| acc + x.sum_metadata())
    }

    pub fn value(&self) -> usize {
        if self.children.len() == 0 {
            self.sum_metadata()
        } else {
            self.metadata
                .iter()
                .filter_map(|metadata| self.children.get(metadata - 1))
                .fold(0, |acc, x| acc + x.value())
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Box<Node> {
    Box::new(Node::from_flat(
        &mut input.split_whitespace().map(|s| s.parse().unwrap()),
    ))
}

#[aoc(day8, part1)]
pub fn solve_part1(root: &Node) -> usize {
    root.sum_metadata()
}

#[aoc(day8, part2)]
pub fn solve_part2(root: &Node) -> usize {
    root.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 138)
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 66)
    }
}
