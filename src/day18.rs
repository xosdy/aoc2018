use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
    Open,
    Trees,
    Lumberyard,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "." => Tile::Open,
            "|" => Tile::Trees,
            "#" => Tile::Lumberyard,
            _ => unreachable!(),
        })
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Tile::Open => ".",
            Tile::Trees => "|",
            Tile::Lumberyard => "#",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid(Vec<Vec<Tile>>);

impl Grid {
    pub fn resource_value(&self) -> usize {
        self.tile_count(&Tile::Trees) * self.tile_count(&Tile::Lumberyard)
    }

    pub fn tile_count(&self, tile_type: &Tile) -> usize {
        self.0.iter().flatten().filter(|&t| t == tile_type).count()
    }

    pub fn run(&mut self, times: usize) {
        let mut snapshots = vec![];

        for minutes in 1..=times {
            snapshots.push(self.clone());
            self.tick();

            if let Some((first_time, _)) = snapshots.iter().enumerate().find(|&(_, g)| g == self) {
                let interval = minutes - first_time;
                self.0 = snapshots
                    .remove(first_time + (times - first_time) % interval)
                    .0;
                break;
            }
        }
    }

    pub fn tick(&mut self) {
        let mut new_grid = self.0.clone();
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                let stats = self.adjacent(x, y);
                match self.0[y][x] {
                    Tile::Open => {
                        if *stats.get(&Tile::Trees).unwrap_or(&0) >= 3 {
                            new_grid[y][x] = Tile::Trees;
                        }
                    }
                    Tile::Trees => {
                        if *stats.get(&Tile::Lumberyard).unwrap_or(&0) >= 3 {
                            new_grid[y][x] = Tile::Lumberyard;
                        }
                    }
                    Tile::Lumberyard => {
                        if *stats.get(&Tile::Lumberyard).unwrap_or(&0) == 0
                            || *stats.get(&Tile::Trees).unwrap_or(&0) == 0
                        {
                            new_grid[y][x] = Tile::Open;
                        }
                    }
                }
            }
        }

        self.0 = new_grid;
    }

    fn adjacent(&self, center_x: usize, center_y: usize) -> HashMap<Tile, usize> {
        let mut stats: HashMap<Tile, usize> = HashMap::new();
        for y in center_y.saturating_sub(1)..=center_y + 1 {
            for x in center_x.saturating_sub(1)..=center_x + 1 {
                if x == center_x && y == center_y {
                    continue;
                }

                if y < self.0.len() && x < self.0[0].len() {
                    stats
                        .entry(self.0[y][x].clone())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }
        stats
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_string().parse().unwrap())
                        .collect()
                })
                .collect(),
        ))
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                write!(f, "{}", self.0[y][x])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Grid {
    input.parse().unwrap()
}

#[aoc(day18, part1)]
pub fn solve_part1(grid: &Grid) -> usize {
    let mut grid = grid.to_owned();
    grid.run(10);
    grid.resource_value()
}

#[aoc(day18, part2)]
pub fn solve_part2(grid: &Grid) -> usize {
    let mut grid = grid.to_owned();
    grid.run(1000000000);

    grid.resource_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn part1() {
        let mut grid = input_generator(TEST_INPUT);
        grid.run(10);
        assert_eq!(grid.resource_value(), 1147);
    }
}
