use cgmath::Vector2;
use regex::Regex;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    position: Vector2<i32>,
    velocity: Vector2<i32>,
}

#[derive(Debug)]
pub struct Grid(Vec<Point>);

impl Grid {
    pub fn new(points: &Vec<Point>) -> Grid {
        Grid(points.clone())
    }

    pub fn update(&mut self) {
        let mut new_grid = Vec::new();
        for p in self.0.iter() {
            let mut p = p.clone();
            p.position += p.velocity;
            new_grid.push(p);
        }

        *self = Grid(new_grid);
    }

    pub fn size(&self) -> (Vector2<i32>, Vector2<i32>) {
        let min_x = self.0.iter().map(|p| p.position.x).min().unwrap();
        let max_x = self.0.iter().map(|p| p.position.x).max().unwrap();
        let min_y = self.0.iter().map(|p| p.position.y).min().unwrap();
        let max_y = self.0.iter().map(|p| p.position.y).max().unwrap();

        (
            Vector2 { x: min_x, y: min_y },
            Vector2 {
                x: max_x - min_x + 1,
                y: max_y - min_y + 1,
            },
        )
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let delta = self.size();
        let mut s = String::new();

        for y in delta.0.y..delta.0.y + delta.1.y {
            for x in delta.0.x..delta.0.x + delta.1.x {
                if self.0.iter().any(|p| p.position == Vector2 { x, y }) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s.trim())
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Point> {
    let re = Regex::new(r"-?\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let result: Vec<_> = re
                .captures_iter(line)
                .map(|cap| cap[0].parse().unwrap())
                .collect();

            Point {
                position: Vector2 {
                    x: result[0],
                    y: result[1],
                },
                velocity: Vector2 {
                    x: result[2],
                    y: result[3],
                },
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(points: &Vec<Point>) -> String {
    let mut grid = Grid::new(points);
    loop {
        let height = grid.size().1.y;
        if height == 10 {
            break;
        }

        grid.update();
    }

    format!("\n{}", grid)
}
