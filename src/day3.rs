use std::collections::HashMap;
use std::collections::HashSet;

pub struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(PartialEq)]
pub enum Status {
    Valid(u32),
    Overlap(Vec<u32>),
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|line| {
            // Claim example
            //  id    x y  w h
            // #123 @ 3,2: 5x4
            let sharp_index = line.find('#').unwrap();
            let at_index = line.find('@').unwrap();
            let id = line[sharp_index + 1..at_index - 1].parse().unwrap();
            let colon_index = line.find(':').unwrap();
            let mut offset = line[at_index + 2..colon_index]
                .split(',')
                .map(|n| n.parse().unwrap());
            let mut size = line[colon_index + 2..]
                .split('x')
                .map(|n| n.parse().unwrap());
            Claim {
                id,
                x: offset.next().unwrap(),
                y: offset.next().unwrap(),
                width: size.next().unwrap(),
                height: size.next().unwrap(),
            }
        })
        .collect()
}

fn gen_grid(claims: &Vec<Claim>) -> HashMap<u32, Status> {
    let grid_width = claims
        .iter()
        .max_by_key(|claim| claim.x + claim.width)
        .unwrap();
    let grid_width = grid_width.x + grid_width.width;
    let get_position = |x: u32, y: u32| y * grid_width + x;

    let mut grid = HashMap::<u32, Status>::new();
    for claim in claims.iter() {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                let position = get_position(x, y);
                grid.entry(position)
                    .and_modify(|e| match e {
                        Status::Valid(id) => *e = Status::Overlap(vec![*id, claim.id]),
                        Status::Overlap(ids) => ids.push(claim.id),
                    })
                    .or_insert(Status::Valid(claim.id));
            }
        }
    }

    grid
}

#[aoc(day3, part1)]
pub fn solve_part1(claims: &Vec<Claim>) -> usize {
    let grid = gen_grid(claims);

    grid.iter()
        .filter(|(_, x)| match x {
            Status::Valid(_) => false,
            Status::Overlap(_) => true,
        })
        .count()
}

#[aoc(day3, part2)]
pub fn solve_part2(claims: &Vec<Claim>) -> u32 {
    let grid = gen_grid(claims);

    let mut ids: HashSet<u32> = claims.iter().map(|claim| claim.id).collect();

    grid.iter()
        .filter_map(|(_, x)| match x {
            Status::Valid(_) => None,
            Status::Overlap(v) => Some(v),
        })
        .for_each(|v| {
            v.iter().for_each(|id| {
                ids.remove(id);
            })
        });

    *ids.iter().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator(
                r"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
            )),
            4
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve_part2(&input_generator(
                r"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"
            )),
            3
        );
    }
}
