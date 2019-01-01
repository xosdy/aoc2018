use na::Vector4;
use std::mem::replace;

pub fn distance(p1: Vector4<i32>, p2: Vector4<i32>) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs() + (p1.w - p2.w).abs()
}

pub fn can_merge(c1: &[Vector4<i32>], c2: &[Vector4<i32>]) -> bool {
    for &p1 in c1 {
        for &p2 in c2 {
            if distance(p1, p2) <= 3 {
                return true;
            }
        }
    }

    false
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<Vector4<i32>> {
    input
        .lines()
        .map(|line| {
            let coordinate: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
            Vector4::new(coordinate[0], coordinate[1], coordinate[2], coordinate[3])
        })
        .collect()
}

#[aoc(day25, part1)]
pub fn solve_part1(points: &[Vector4<i32>]) -> usize {
    let mut constellations: Vec<Option<Vec<Vector4<i32>>>> =
        points.iter().map(|&p| Some(vec![p])).collect();

    loop {
        let mut has_merged = false;
        for i in 0..constellations.len() {
            if constellations[i].is_none() {
                continue;
            }

            for j in i + 1..constellations.len() {
                if constellations[j].is_none() {
                    continue;
                }

                if can_merge(
                    constellations[i].as_ref().unwrap(),
                    constellations[j].as_ref().unwrap(),
                ) {
                    let tmp_points = replace(&mut constellations[j], None).unwrap();
                    constellations[i].as_mut().unwrap().extend(tmp_points);
                    has_merged = true;
                }
            }
        }

        constellations.retain(|c| c.is_some());

        if !has_merged {
            break;
        }
    }

    constellations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: &[(&str, usize)] = &[
        (
            r"0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0",
            2,
        ),
        (
            r"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
",
            4,
        ),
        (
            r"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2",
            3,
        ),
        (
            r"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2",
            8,
        ),
    ];

    #[test]
    fn part1() {
        for &(input, expect) in TEST_CASES {
            let points = input_generator(input);
            assert_eq!(solve_part1(&points), expect);
        }
    }
}
