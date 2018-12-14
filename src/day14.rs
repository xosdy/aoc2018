pub struct Recipes {
    elves: Vec<usize>,
    scores: Vec<u32>,
}

impl Recipes {
    pub fn new() -> Recipes {
        Recipes {
            elves: vec![0, 1],
            scores: vec![3, 7],
        }
    }

    pub fn tick(&mut self) {
        let new_recipes: u32 = self.elves.iter().map(|&e| self.scores[e]).sum();
        self.scores.extend(
            new_recipes
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap()),
        );

        for e in &mut self.elves {
            *e = (*e + self.scores[*e] as usize + 1) % self.scores.len();
        }
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(recipe_count: &str) -> String {
    let recipe_count = recipe_count.parse().unwrap();
    let mut recipes = Recipes::new();
    while recipes.scores.len() < recipe_count + 10 {
        recipes.tick();
    }

    recipes.scores[recipe_count..recipe_count + 10]
        .iter()
        .map(|&s| std::char::from_digit(s, 10).unwrap())
        .collect()
}

#[aoc(day14, part2)]
pub fn solve_part2(digits: &str) -> usize {
    let digits: Vec<_> = digits
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let mut recipes = Recipes::new();

    loop {
        if recipes.scores.ends_with(&digits) {
            return recipes.scores.len() - digits.len();
        } else if recipes.scores[..recipes.scores.len() - 1].ends_with(&digits) {
            return recipes.scores.len() - digits.len() - 1;
        }

        recipes.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_part1("9"), "5158916779");
        assert_eq!(solve_part1("5"), "0124515891");
        assert_eq!(solve_part1("18"), "9251071085");
        assert_eq!(solve_part1("2018"), "5941429882");
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2("51589"), 9);
        assert_eq!(solve_part2("01245"), 5);
        assert_eq!(solve_part2("92510"), 18);
        assert_eq!(solve_part2("59414"), 2018);
    }
}
