use na::Vector2;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Vec2(Vector2<usize>);

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Vec2 {
        Vec2(Vector2::new(x, y))
    }
}

impl Ord for Vec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0.y, self.0.x).cmp(&(other.0.y, other.0.x))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Sand,
    Clay,
    RestWater,
    FlowingWater,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tile = match self {
            Tile::Sand => '.',
            Tile::Clay => '#',
            Tile::RestWater => '~',
            Tile::FlowingWater => '|',
        };
        write!(f, "{}", tile)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Grid(BTreeMap<Vec2, Tile>);

impl Grid {
    pub fn water_count(&self) -> usize {
        let min_y = (self.0.iter().find(|&(_, t)| *t == Tile::Clay).unwrap().0)
            .0
            .y;
        self.0
            .iter()
            .filter(|&(p, t)| p.0.y >= min_y && (*t == Tile::FlowingWater || *t == Tile::RestWater))
            .count()
    }

    pub fn rest_water_count(&self) -> usize {
        let min_y = (self.0.iter().find(|&(_, t)| *t == Tile::Clay).unwrap().0)
            .0
            .y;
        self.0
            .iter()
            .filter(|&(p, t)| p.0.y >= min_y && *t == Tile::RestWater)
            .count()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Grid = Default::default();
        let re = Regex::new(r"(\w)=(\d+),\s\w=(\d+)\.\.(\d+)").unwrap();
        for cap in re.captures_iter(s) {
            let fixed_coordinate = cap[2].parse().unwrap();
            let range = cap[3].parse().unwrap()..=cap[4].parse().unwrap();

            if &cap[1] == "x" {
                grid.0
                    .extend(range.map(|y| (Vec2::new(fixed_coordinate, y), Tile::Clay)));
            } else {
                grid.0
                    .extend(range.map(|x| (Vec2::new(x, fixed_coordinate), Tile::Clay)));
            }
        }

        Ok(grid)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=(self.0.iter().next_back().unwrap().0).0.y {
            for x in 450..=550 {
                let tile = self.0.get(&Vec2(Vector2::new(x, y))).unwrap_or(&Tile::Sand);
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

pub enum WaterDirection {
    Both,
    Left,
    Right,
}

pub fn fill(grid: &mut Grid, position: &Vec2, direction: WaterDirection) -> Option<usize> {
    let max_y = (grid.0.iter().next_back().unwrap().0).0.y;
    if position.0.y > max_y {
        return None;
    }

    match grid.0.get(position).unwrap_or(&Tile::Sand) {
        Tile::Sand => {
            grid.0.insert(position.clone(), Tile::FlowingWater);
            fill(
                grid,
                &Vec2::new(position.0.x, position.0.y + 1),
                WaterDirection::Both,
            )?;
            match direction {
                WaterDirection::Both => match (
                    fill(
                        grid,
                        &Vec2::new(position.0.x - 1, position.0.y),
                        WaterDirection::Left,
                    ),
                    fill(
                        grid,
                        &Vec2::new(position.0.x + 1, position.0.y),
                        WaterDirection::Right,
                    ),
                ) {
                    (Some(l), Some(r)) => {
                        (l + 1..r).for_each(|x| {
                            grid.0.insert(Vec2::new(x, position.0.y), Tile::RestWater);
                        });
                        Some(position.0.x)
                    }
                    _ => None,
                },
                WaterDirection::Left => fill(
                    grid,
                    &Vec2::new(position.0.x - 1, position.0.y),
                    WaterDirection::Left,
                ),
                WaterDirection::Right => fill(
                    grid,
                    &Vec2::new(position.0.x + 1, position.0.y),
                    WaterDirection::Right,
                ),
            }
        }
        Tile::Clay | Tile::RestWater => Some(position.0.x),
        Tile::FlowingWater => None,
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Grid {
    input.parse().unwrap()
}

#[aoc(day17, part1)]
pub fn solve_part1(grid: &Grid) -> usize {
    let mut grid = grid.to_owned();
    fill(&mut grid, &Vec2::new(500, 0), WaterDirection::Both);
    grid.water_count()
}

#[aoc(day17, part2)]
pub fn solve_part2(grid: &Grid) -> usize {
    let mut grid = grid.to_owned();
    fill(&mut grid, &Vec2::new(500, 0), WaterDirection::Both);
    grid.rest_water_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn part1() {
        let mut grid = input_generator(TEST_INPUT);
        fill(&mut grid, &Vec2::new(500, 0), WaterDirection::Both);
        assert_eq!(grid.water_count(), 57);
    }
}
