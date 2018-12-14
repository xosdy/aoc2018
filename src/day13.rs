use na::{Matrix2, Vector2};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

type Grid = Vec<Vec<Tile>>;

lazy_static! {
    static ref TURN_LEFT: Matrix2<i32> = Matrix2::new(0, 1, -1, 0);
    static ref GO_STRAIGHT: Matrix2<i32> = Matrix2::new(1, 0, 0, 1);
    static ref TURN_RIGHT: Matrix2<i32> = Matrix2::new(0, -1, 1, 0);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tile {
    None,
    Horizontal,
    Vertical,
    PositiveSlope,
    NegativeSlope,
    Intersection,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cart {
    position: Vector2<i32>,
    direction: Vector2<i32>,
    next_rotation: Matrix2<i32>,
    crashed: bool,
}

impl Cart {
    pub fn new(position: Vector2<i32>, direction: Vector2<i32>) -> Cart {
        Cart {
            position,
            direction,
            next_rotation: *TURN_LEFT,
            crashed: false,
        }
    }

    pub fn set_next_direction(&mut self) {
        self.direction = self.next_rotation * self.direction;

        if self.next_rotation == *TURN_LEFT {
            self.next_rotation = *GO_STRAIGHT;
        } else if self.next_rotation == *GO_STRAIGHT {
            self.next_rotation = *TURN_RIGHT
        } else if self.next_rotation == *TURN_RIGHT {
            self.next_rotation = *TURN_LEFT
        }
    }

    pub fn tick(&mut self, grid: &Grid) {
        self.position += self.direction;

        let tile = &grid[self.position.y as usize][self.position.x as usize];
        match tile {
            Tile::Horizontal | Tile::Vertical => {}
            Tile::PositiveSlope => {
                self.direction = Vector2::new(self.direction.y, self.direction.x)
            }
            Tile::NegativeSlope => {
                self.direction = Vector2::new(-self.direction.y, -self.direction.x)
            }
            Tile::Intersection => self.set_next_direction(),
            _ => unreachable!("Cart not in a track"),
        };
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position
            .y
            .cmp(&other.position.y)
            .then(self.position.x.cmp(&other.position.x))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct System {
    grid: Grid,
    carts: Vec<Cart>,
}

impl System {
    pub fn run_until_first_crash(&mut self) -> Vector2<i32> {
        loop {
            let crashed_carts = self.tick();
            if let Some(pos) = crashed_carts.get(0) {
                return *pos;
            }
        }
    }

    pub fn run_until_one_cart(&mut self) -> Vector2<i32> {
        while self.carts.len() > 1 {
            self.tick();
        }

        self.carts[0].position
    }

    pub fn tick(&mut self) -> Vec<Vector2<i32>> {
        self.carts.sort();

        for i in 0..self.carts.len() {
            let cart = &mut self.carts[i];
            cart.tick(&self.grid);

            self.mark_collision();
        }

        let crashed_carts = self
            .carts
            .iter()
            .filter(|cart| cart.crashed)
            .map(|cart| cart.position)
            .collect();
        self.carts.retain(|cart| !cart.crashed);

        crashed_carts
    }

    pub fn mark_collision(&mut self) {
        let mut set = HashSet::new();
        let mut colliding_at = HashSet::new();
        for cart in &self.carts {
            if !set.insert(cart.position) {
                colliding_at.insert(cart.position);
            }
        }

        self.carts
            .iter_mut()
            .filter(|cart| colliding_at.contains(&cart.position))
            .for_each(|cart| cart.crashed = true);
    }
}

impl FromStr for System {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut carts = Vec::new();
        let mut grid: Vec<Vec<Tile>> = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, tile)| match tile {
                        ' ' => Tile::None,
                        '-' => Tile::Horizontal,
                        '|' => Tile::Vertical,
                        '/' => Tile::NegativeSlope,
                        '\\' => Tile::PositiveSlope,
                        '+' => Tile::Intersection,
                        '^' => {
                            carts.push(Cart::new(
                                Vector2::new(x as i32, y as i32),
                                Vector2::new(0, -1),
                            ));
                            Tile::None
                        }
                        'v' => {
                            carts.push(Cart::new(
                                Vector2::new(x as i32, y as i32),
                                Vector2::new(0, 1),
                            ));
                            Tile::None
                        }
                        '<' => {
                            carts.push(Cart::new(
                                Vector2::new(x as i32, y as i32),
                                Vector2::new(-1, 0),
                            ));
                            Tile::None
                        }
                        '>' => {
                            carts.push(Cart::new(
                                Vector2::new(x as i32, y as i32),
                                Vector2::new(1, 0),
                            ));
                            Tile::None
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        for cart in &carts {
            if grid[cart.position.y as usize][cart.position.x as usize - 1] != Tile::None
                && grid[cart.position.y as usize][cart.position.x as usize + 1] != Tile::None
            {
                grid[cart.position.y as usize][cart.position.x as usize] = Tile::Horizontal;
            } else if grid[cart.position.y as usize - 1][cart.position.x as usize] != Tile::None
                && grid[cart.position.y as usize + 1][cart.position.x as usize] != Tile::None
            {
                grid[cart.position.y as usize][cart.position.x as usize] = Tile::Vertical;
            } else {
                dbg!(&grid);
                dbg!(&cart);
                unimplemented!();
            }
        }

        Ok(System { grid, carts })
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        Tile::None => ' ',
                        Tile::Horizontal => '-',
                        Tile::Vertical => '|',
                        Tile::PositiveSlope => '\\',
                        Tile::NegativeSlope => '/',
                        Tile::Intersection => '+',
                    })
                    .collect()
            })
            .collect();

        for cart in &self.carts {
            lines[cart.position.y as usize].replace_range(
                cart.position.x as usize..cart.position.x as usize + 1,
                if cart.direction == Vector2::new(0, -1) {
                    "^"
                } else if cart.direction == Vector2::new(0, 1) {
                    "v"
                } else if cart.direction == Vector2::new(1, 0) {
                    ">"
                } else if cart.direction == Vector2::new(-1, 0) {
                    "<"
                } else {
                    unreachable!()
                },
            );
        }

        write!(f, "{}", lines.join("\n"))
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> System {
    input.parse().unwrap()
}

#[aoc(day13, part1)]
pub fn solve_part1(system: &System) -> String {
    let pos = system.clone().run_until_first_crash();
    format!("{},{}", pos.x, pos.y)
}

#[aoc(day13, part2)]
pub fn solve_part2(system: &System) -> String {
    let pos = system.clone().run_until_one_cart();
    format!("{},{}", pos.x, pos.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut system = input_generator(include_str!("../tests/day13_part1.txt"));
        assert_eq!(system.run_until_first_crash(), Vector2::new(7, 3));
    }

    #[test]
    fn part2() {
        let mut system = input_generator(include_str!("../tests/day13_part2.txt"));
        assert_eq!(system.run_until_one_cart(), Vector2::new(6, 4));
    }
}
