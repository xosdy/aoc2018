use std::collections::HashMap;

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
    Overlap,
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

#[aoc(day3, part1)]
pub fn solve_part1(claims: &Vec<Claim>) -> usize {
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
                if grid.contains_key(&position) {
                    grid.insert(position, Status::Overlap);
                } else {
                    grid.insert(position, Status::Valid(claim.id));
                }
            }
        }
    }

    grid.iter()
        .filter(|(_, ref x)| **x == Status::Overlap)
        .count()
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
}
