use cgmath::Vector2;
use std::collections::HashMap;

pub fn get_cell_power(serial: u32, position: &Vector2<u32>) -> i32 {
    let rack_id = position.x + 10;
    let power_level = (((rack_id * position.y + serial) * rack_id) / 100 % 10) as i32;
    power_level - 5
}

pub fn get_square_power(serial: u32, position: &Vector2<u32>, size: &Vector2<u32>) -> i32 {
    let mut total_power = 0;
    for y in position.y..position.y + size.y {
        for x in position.x..position.x + size.x {
            total_power += get_cell_power(serial, &Vector2 { x, y });
        }
    }

    total_power
}

pub fn find_largest_power(serial: u32) -> (Vector2<u32>, i32) {
    let mut powers = HashMap::new();
    for y in 1..=298 {
        for x in 1..=298 {
            let position = Vector2 { x, y };
            let power = get_square_power(serial, &position, &Vector2 { x: 3, y: 3 });
            powers.insert(position, power);
        }
    }

    powers.into_iter().max_by_key(|&(_, p)| p).unwrap()
}

/// Use [Summed-area table](https://en.wikipedia.org/wiki/Summed-area_table) to calculate largest square
pub fn find_largest_power_any_square(serial: u32) -> (Vector2<u32>, usize, i32) {
    let mut sum = [[0; 301]; 301];
    for y in 1..=300 {
        for x in 1..=300 {
            let position = Vector2 {
                x: x as u32,
                y: y as u32,
            };
            let power = get_cell_power(serial, &position);

            sum[y][x] = power + sum[y - 1][x] + sum[y][x - 1] - sum[y - 1][x - 1];
        }
    }

    let mut best = (Vector2 { x: 0, y: 0 }, 0, i32::min_value());
    for size in 1..=300 {
        for y in size..=300 {
            for x in size..=300 {
                let position = Vector2 {
                    x: x as u32,
                    y: y as u32,
                };
                let power =
                    sum[y - size][x - size] + sum[y][x] - sum[y][x - size] - sum[y - size][x];

                if power > best.2 {
                    best = (
                        // Need top-left position
                        position
                            - Vector2 {
                                x: size as u32 - 1,
                                y: size as u32 - 1,
                            },
                        size,
                        power,
                    );
                }
            }
        }
    }

    best
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Box<u32> {
    Box::new(input.trim().parse().unwrap())
}

#[aoc(day11, part1)]
pub fn solve_part1(serial: &u32) -> String {
    let pos = find_largest_power(*serial).0;
    format!("{},{}", pos.x, pos.y)
}

#[aoc(day11, part2)]
pub fn solve_part2(serial: &u32) -> String {
    let (pos, size, _) = find_largest_power_any_square(*serial);
    format!("{},{},{}", pos.x, pos.y, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cell_power() {
        assert_eq!(get_cell_power(57, &Vector2 { x: 122, y: 79 }), -5);
        assert_eq!(get_cell_power(39, &Vector2 { x: 217, y: 196 }), 0);
        assert_eq!(get_cell_power(71, &Vector2 { x: 101, y: 153 }), 4);
    }

    #[test]
    fn test_find_largest_power() {
        assert_eq!(find_largest_power(18), (Vector2 { x: 33, y: 45 }, 29));
        assert_eq!(find_largest_power(42), (Vector2 { x: 21, y: 61 }, 30));
    }

    #[test]
    fn test_find_largest_power_any_square() {
        assert_eq!(
            find_largest_power_any_square(18),
            (Vector2 { x: 90, y: 269 }, 16, 113)
        );
        assert_eq!(
            find_largest_power_any_square(42),
            (Vector2 { x: 232, y: 251 }, 12, 119)
        )
    }
}
