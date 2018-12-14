#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day14, part1)]
pub fn solve_part1(recipe_count: &usize) -> String {
    let mut elves = vec![0, 1];
    let mut scores = vec![3, 7];

    while scores.len() < recipe_count + 10 {
        let new_recipes: u32 = elves.iter().map(|&e| scores[e]).sum();
        scores.extend(
            new_recipes
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap()),
        );

        elves
            .iter_mut()
            .for_each(|e| *e = (*e + scores[*e] as usize + 1) % scores.len());
    }

    scores[*recipe_count..*recipe_count + 10]
        .iter()
        .map(|&s| std::char::from_digit(s, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&9), "5158916779");
        assert_eq!(solve_part1(&5), "0124515891");
        assert_eq!(solve_part1(&18), "9251071085");
        assert_eq!(solve_part1(&2018), "5941429882");
    }
}
