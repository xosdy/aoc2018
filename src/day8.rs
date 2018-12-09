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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator(r"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")),
            138
        )
    }
}
