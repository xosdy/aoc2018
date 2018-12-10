pub struct Game {
    players: usize,
    last_marble: u32,
}

impl Game {
    pub fn run(&self) -> u32 {
        let mut scores: Vec<u32> = vec![0; self.players];

        let mut current_marble_index: isize = 0;
        let mut marbles = vec![0];
        for (marble, player_index) in (1..=self.last_marble).zip((0..scores.len()).cycle()) {
            if marble % 23 != 0 {
                current_marble_index += 2;
                if current_marble_index > marbles.len() as isize {
                    current_marble_index -= marbles.len() as isize;
                }

                marbles.insert(current_marble_index as usize, marble);
            } else {
                current_marble_index -= 7;
                if current_marble_index < 0 {
                    current_marble_index += marbles.len() as isize;
                }

                scores[player_index] += marble;
                scores[player_index] += marbles.remove(current_marble_index as usize);
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
