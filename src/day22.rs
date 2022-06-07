use na::Vector2;
use num_derive::FromPrimitive;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::EnumCount;

#[derive(Debug, Clone, Copy, EnumCount, FromPrimitive)]
pub enum RegionType {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Hash)]
pub enum Tool {
    Neither = 0,
    Torch = 1,
    ClimbingGear = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PositionAndTool {
    position: Vector2<usize>,
    tool: Tool,
}

#[derive(Debug)]
pub struct MinScored(usize, PositionAndTool);

impl Ord for MinScored {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl PartialOrd for MinScored {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MinScored {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for MinScored {}

#[derive(Debug, Clone)]
pub struct Cave {
    depth: usize,
    target: Vector2<usize>,
    geologic_index_cache: HashMap<Vector2<usize>, usize>,
}

impl Cave {
    pub fn new(depth: usize, target: Vector2<usize>) -> Cave {
        Cave {
            depth,
            target,
            geologic_index_cache: HashMap::new(),
        }
    }

    pub fn geologic_index(&mut self, region: Vector2<usize>) -> usize {
        if let Some(&index) = self.geologic_index_cache.get(&region) {
            return index;
        }

        let index = if (region.x == 0 && region.y == 0) || region == self.target {
            0
        } else if region.y == 0 {
            region.x * 16807
        } else if region.x == 0 {
            region.y * 48271
        } else {
            self.erosion_level(Vector2::new(region.x - 1, region.y))
                * self.erosion_level(Vector2::new(region.x, region.y - 1))
        };

        self.geologic_index_cache.insert(region, index);
        index
    }

    pub fn erosion_level(&mut self, region: Vector2<usize>) -> usize {
        (self.geologic_index(region) + self.depth) % 20183
    }

    pub fn region_type(&mut self, region: Vector2<usize>) -> RegionType {
        num_traits::FromPrimitive::from_usize(self.erosion_level(region) % RegionType::COUNT)
            .unwrap()
    }

    pub fn risk_level(&mut self, region: Vector2<usize>) -> usize {
        let mut sum = 0;
        for y in 0..=region.y {
            for x in 0..=region.x {
                sum += self.region_type(Vector2::new(x, y)) as usize;
            }
        }

        sum
    }

    pub fn spent_minutes(&mut self) -> usize {
        let mut visited = HashSet::new();
        let mut visit_next = BinaryHeap::new();
        visit_next.push(MinScored(
            0,
            PositionAndTool {
                position: Vector2::zeros(),
                tool: Tool::Torch,
            },
        ));

        let target = PositionAndTool {
            position: self.target,
            tool: Tool::Torch,
        };

        while let Some(MinScored(minutes, current)) = visit_next.pop() {
            // println!("{} {} {} {:?}", minutes, current.position.x, current.position.y, current.tool);
            if visited.contains(&current) {
                continue;
            }

            if current == target {
                return minutes;
            }

            for tool in Tool::iter() {
                if tool != current.tool
                    && tool as usize != self.region_type(current.position) as usize
                {
                    visit_next.push(MinScored(
                        minutes + 7,
                        PositionAndTool {
                            position: current.position,
                            tool,
                        },
                    ));
                }
            }

            for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_x = current.position.x as i64 + dx;
                let new_y = current.position.y as i64 + dy;
                if new_x < 0 || new_y < 0 {
                    continue;
                }

                let position = Vector2::new(new_x as usize, new_y as usize);
                if self.region_type(position) as usize == current.tool as usize {
                    continue;
                }

                visit_next.push(MinScored(
                    minutes + 1,
                    PositionAndTool {
                        position,
                        tool: current.tool,
                    },
                ));
            }

            visited.insert(current);
        }

        unreachable!()
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Cave {
    let mut iter = input.lines();
    let depth = iter
        .next()
        .and_then(|s| s.split_whitespace().next_back())
        .and_then(|depth_str| depth_str.parse().ok())
        .unwrap();
    let target = iter
        .next()
        .and_then(|s| s.split_whitespace().next_back())
        .and_then(|p| {
            let xy: Vec<_> = p.split(',').map(|x| x.parse().unwrap()).collect();
            Some(Vector2::new(xy[0], xy[1]))
        })
        .unwrap();

    Cave::new(depth, target)
}

#[aoc(day22, part1)]
pub fn solve_part1(cave: &Cave) -> usize {
    let mut cave = cave.to_owned();
    cave.risk_level(cave.target)
}

#[aoc(day22, part2)]
pub fn solve_part2(cave: &Cave) -> usize {
    let mut cave = cave.to_owned();
    cave.spent_minutes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut cave = Cave::new(510, Vector2::new(10, 10));
        assert_eq!(cave.risk_level(cave.target), 114);
    }

    #[test]
    fn part2() {
        let mut cave = Cave::new(510, Vector2::new(10, 10));
        assert_eq!(cave.spent_minutes(), 45);
    }
}
