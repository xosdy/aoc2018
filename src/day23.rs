use na::Vector3;
use regex::Regex;
use std::collections::HashMap;

pub struct Nanobot {
    position: Vector3<i64>,
    radius: u64,
}

pub fn distance(me: Vector3<i64>, other: Vector3<i64>) -> u64 {
    ((me.x - other.x).abs() + (me.y - other.y).abs() + (me.z - other.z).abs()) as u64
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Nanobot> {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let pos: Vec<_> = (1..=3)
                .map(|i| caps.get(i).and_then(|s| s.as_str().parse().ok()).unwrap())
                .collect();

            Nanobot {
                position: Vector3::new(pos[0], pos[1], pos[2]),
                radius: caps.get(4).and_then(|s| s.as_str().parse().ok()).unwrap(),
            }
        })
        .collect()
}

#[aoc(day23, part1)]
pub fn solve_part1(bots: &[Nanobot]) -> usize {
    let max_radius_bot = bots.iter().max_by_key(|bot| bot.radius).unwrap();
    bots.iter()
        .filter(|bot| distance(max_radius_bot.position, bot.position) <= max_radius_bot.radius)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST_INPUT)), 7);
    }
}
