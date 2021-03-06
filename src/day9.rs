use std::collections::VecDeque;

trait Rotatable {
    fn rotate_left(&mut self, count: usize);
    fn rotate_right(&mut self, count: usize);
}

impl<T> Rotatable for VecDeque<T> {
    fn rotate_left(&mut self, count: usize) {
        for _ in 0..count {
            let tmp = self.pop_back().unwrap();
            self.push_front(tmp);
        }
    }

    fn rotate_right(&mut self, count: usize) {
        for _ in 0..count {
            let tmp = self.pop_front().unwrap();
            self.push_back(tmp);
        }
    }
}

#[derive(Clone)]
pub struct Game {
    players: usize,
    last_marble: u32,
}

impl Game {
    pub fn run(&self) -> u32 {
        let mut scores: Vec<u32> = vec![0; self.players];

        let mut marbles = VecDeque::new();
        marbles.push_back(0);
        for (marble, player_index) in (1..=self.last_marble).zip((0..scores.len()).cycle()) {
            if marble % 23 != 0 {
                marbles.rotate_left(2);
                marbles.push_back(marble);
            } else {
                marbles.rotate_right(7);
                scores[player_index] += marble + marbles.pop_back().unwrap();
            }
        }

        *scores.iter().max().unwrap()
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Box<Game> {
    let parts: Vec<_> = input.split_whitespace().collect();
    Box::new(Game {
        players: parts[0].parse().unwrap(),
        last_marble: parts[6].parse().unwrap(),
    })
}

#[aoc(day9, part1)]
pub fn solve_part1(game: &Game) -> u32 {
    game.run()
}

#[aoc(day9, part2)]
pub fn solve_part2(game: &Game) -> u32 {
    let mut game = game.clone();
    game.last_marble *= 100;
    game.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let cases = [
            ("10 players; last marble is worth 1618 points", 8317),
            ("13 players; last marble is worth 7999 points", 146373),
            ("17 players; last marble is worth 1104 points", 2764),
            ("21 players; last marble is worth 6111 points", 54718),
            ("30 players; last marble is worth 5807 points", 37305),
        ];

        for case in &cases {
            assert_eq!(solve_part1(&input_generator(case.0)), case.1);
        }
    }
}
