use na::Vector3;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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

#[aoc(day23, part2)]
pub fn solve_part2(bots: &[Nanobot]) -> u64 {
    let mut pq = BinaryHeap::new();
    for bot in bots {
        let dist = distance(bot.position, Vector3::zeros());
        pq.push((Reverse(dist.saturating_sub(bot.radius)), 1));
        pq.push((Reverse(dist + bot.radius), -1));
    }

    let mut count = 0;
    let mut max_count = 0;
    let mut min_dist = 0;
    while let Some((dist, diff)) = pq.pop() {
        count += diff;

        if count > max_count {
            max_count = count;
            min_dist = dist.0;
        }
    }

    min_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator(
                r"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"
            )),
            7
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(&input_generator(
                r"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"
            )),
            36
        )
    }
}
