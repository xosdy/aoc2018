use na::Vector2;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vector2<i32>> {
    input
        .lines()
        .map(|line| {
            let mut axes = line.split(',');
            Vector2::new(
                axes.next().unwrap().trim().parse().unwrap(),
                axes.next().unwrap().trim().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(points: &Vec<Vector2<i32>>) -> usize {
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x as usize + 1;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y as usize + 1;

    let mut grid: Vec<Vec<Vec<usize>>> = Vec::with_capacity(max_y);
    grid.resize_with(max_y, || {
        let mut v = Vec::with_capacity(max_x);
        v.resize_default(max_x);
        v
    });

    for y in 0..max_y {
        for x in 0..max_x {
            let distances: Vec<_> = points
                .iter()
                .map(|p| (p.x - x as i32).abs() + (p.y - y as i32).abs())
                .collect();
            let min_distance = distances.iter().min().unwrap();
            let closest_points = distances
                .iter()
                .enumerate()
                .filter(|(_, d)| *d == min_distance)
                .map(|(i, _)| i)
                .collect();
            grid[y][x] = closest_points;
        }
    }

    let mut infinite_point_indices = HashSet::new();
    for x in 0..max_x {
        if grid[0][x].len() == 1 {
            infinite_point_indices.insert(grid[0][x][0]);
        }

        if grid[max_y - 1][x].len() == 1 {
            infinite_point_indices.insert(grid[max_y - 1][x][0]);
        }
    }
    for y in 0..max_y {
        if grid[y][0].len() == 1 {
            infinite_point_indices.insert(grid[y][0][0]);
        }

        if grid[y][max_x - 1].len() == 1 {
            infinite_point_indices.insert(grid[y][max_x - 1][0]);
        }
    }

    let mut finite_point_count: HashMap<usize, usize> = HashMap::new();
    for y in 1..max_y - 1 {
        for x in 1..max_x - 1 {
            let points = &grid[y][x];
            if points.len() == 1 && !infinite_point_indices.contains(&points[0]) {
                finite_point_count
                    .entry(points[0])
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    *finite_point_count.iter().max_by_key(|&(_, v)| v).unwrap().1
}

fn in_distance_count(points: &Vec<Vector2<i32>>, limit: usize) -> usize {
    let offset = (limit / points.len()) as i32;

    let min_x = points.iter().min_by_key(|p| p.x).unwrap().x - offset;
    let min_y = points.iter().min_by_key(|p| p.y).unwrap().y - offset;
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x + offset;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y + offset;

    (min_x..max_x)
        .flat_map(move |x| {
            (min_y..max_y).map(move |y| {
                points
                    .iter()
                    .map(move |p| (p.x - x).abs() + (p.y - y).abs())
                    .sum::<i32>()
            })
        })
        .filter(|d| *d < limit as i32)
        .count()
}

#[aoc(day6, part2)]
pub fn solve_part2(points: &Vec<Vector2<i32>>) -> usize {
    in_distance_count(points, 10000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator(
                r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
            )),
            17
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            in_distance_count(
                &input_generator(
                    r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
                ),
                32
            ),
            16
        );
    }
}
