use na::Vector2;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Door,
    Room,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Tile::Wall => "#",
            Tile::Door => "+",
            Tile::Room => " ",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Grid(HashMap<Vector2<i32>, Tile>);

impl Grid {
    pub fn pass_most_door(&self) -> usize {
        let distances = self.get_distances(Vector2::zeros());
        let d = distances
            .iter()
            .filter(|(p, _)| self.0.get(p) == Some(&Tile::Room))
            .map(|(_, d)| d)
            .max()
            .unwrap();
        d / 2
    }

    fn add_room(&mut self, mut current: Vector2<i32>, direction: Vector2<i32>) -> Vector2<i32> {
        current += direction;
        self.0.insert(current, Tile::Door);
        current += direction;
        self.0.insert(current, Tile::Room);

        current
    }

    fn get_distances(&self, start: Vector2<i32>) -> HashMap<Vector2<i32>, usize> {
        let mut distances = HashMap::new();
        distances.insert(start, 0);

        let mut open_set = VecDeque::new();
        open_set.push_back(start);
        let mut closed_set = HashSet::new();

        while let Some(p) = open_set.pop_front() {
            let successors = self
                .get_adjacent_tiles(p)
                .into_iter()
                .filter(|&c| self.is_empty(c));
            for s in successors {
                if closed_set.contains(&s) || open_set.contains(&s) {
                    continue;
                }

                open_set.push_back(s);
                distances.insert(s, distances[&p] + 1);
            }

            closed_set.insert(p);
        }

        distances
    }

    fn get_adjacent_tiles(&self, position: Vector2<i32>) -> Vec<Vector2<i32>> {
        vec![
            Vector2::new(position.x, position.y - 1),
            Vector2::new(position.x - 1, position.y),
            Vector2::new(position.x + 1, position.y),
            Vector2::new(position.x, position.y + 1),
        ]
    }

    fn is_empty(&self, position: Vector2<i32>) -> bool {
        self.0.contains_key(&position)
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..s.len() - 1];
        let mut grid: Self = Default::default();
        let mut current = Vector2::zeros();
        grid.0.insert(current, Tile::Room);
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                'E' => current = grid.add_room(current, Vector2::new(1, 0)),
                'S' => current = grid.add_room(current, Vector2::new(0, -1)),
                'W' => current = grid.add_room(current, Vector2::new(-1, 0)),
                'N' => current = grid.add_room(current, Vector2::new(0, 1)),
                '(' => stack.push(current),
                ')' => current = stack.pop().expect("Parentheses not match"),
                '|' => current = *stack.last().unwrap(),
                _ => unreachable!(),
            }
        }

        Ok(grid)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_x = self.0.iter().map(|(p, _)| p.x).min().unwrap();
        let max_x = self.0.iter().map(|(p, _)| p.x).max().unwrap();
        let min_y = self.0.iter().map(|(p, _)| p.y).min().unwrap();
        let max_y = self.0.iter().map(|(p, _)| p.y).max().unwrap();

        for y in (min_y - 1..=max_y + 1).rev() {
            for x in min_x - 1..=max_x + 1 {
                write!(
                    f,
                    "{}",
                    self.0.get(&Vector2::new(x, y)).unwrap_or(&Tile::Wall)
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Grid {
    input.trim().parse().unwrap()
}

#[aoc(day20, part1)]
pub fn solve_part1(grid: &Grid) -> usize {
    grid.pass_most_door()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUTS: &[&(&str, usize)] = &[
        &("^WNE$", 3),
        &("^ENWWW(NEEE|SSE(EE|N))$", 10),
        &("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18),
        &("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23),
        &(
            "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
            31,
        ),
    ];

    #[test]
    fn part1() {
        for &&(input, expect) in TEST_INPUTS {
            let grid: Grid = input.parse().unwrap();
            assert_eq!(grid.pass_most_door(), expect);
        }
    }
}
