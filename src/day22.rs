use na::Vector2;
use std::collections::HashMap;
use strum::EnumCount;

#[derive(EnumCount, FromPrimitive)]
pub enum RegionType {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

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

        let index = if region.x == 0 && region.y == 0 {
            0
        } else if region == self.target {
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
        num_traits::FromPrimitive::from_usize(self.erosion_level(region) % RegionType::count())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut cave = Cave::new(510, Vector2::new(10, 10));
        assert_eq!(cave.risk_level(cave.target), 114);
    }
}
