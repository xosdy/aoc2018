use na::Vector2;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Cavern,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Race {
    Goblin,
    Elf,
}

#[derive(Debug, Clone)]
pub struct Scene {
    grid: Vec<Vec<Tile>>,
    entities: Vec<Entity>,
}

impl Scene {
    pub fn tick(&mut self) -> bool {
        let mut full_round = true;
        for i in 0..self.entities.len() {
            if !self.entities[i].is_alive() {
                continue;
            }

            if self.get_all_enemies(&self.entities[i]).is_empty() {
                full_round = false;
                break;
            }

            if let Some(enemy_index) = self.get_adjacent_enemy(&self.entities[i]) {
                self.attack(i, enemy_index);
                continue;
            }

            let next_step = match self.next_step(i) {
                Some(p) => p,
                None => continue,
            };
            self.entities[i].position = next_step;

            if let Some(enemy_index) = self.get_adjacent_enemy(&self.entities[i]) {
                self.attack(i, enemy_index);
            }
        }

        self.entities.retain(|e| e.is_alive());
        self.entities
            .sort_by(|a, b| (a.position.y, a.position.x).cmp(&(b.position.y, b.position.x)));

        full_round
    }

    fn get_entity_index(&self, position: &Vector2<usize>) -> Option<usize> {
        self.entities.iter().position(|e| e.position == *position)
    }

    fn get_adjacent_enemy(&self, me: &Entity) -> Option<usize> {
        self.get_adjacent_tiles(&me.position)
            .iter()
            .filter_map(|e| self.get_entity_index(e))
            .filter(|&entity_index| {
                self.entities[entity_index].is_alive() && me.is_enemy(&self.entities[entity_index])
            })
            .min_by_key(|&entity_index| self.entities[entity_index].hp)
    }

    fn get_all_enemies(&self, me: &Entity) -> Vec<Vector2<usize>> {
        self.entities
            .iter()
            .filter(|e| e.is_alive() && me.is_enemy(e))
            .map(|e| e.position)
            .collect()
    }

    fn get_adjacent_tiles(&self, position: &Vector2<usize>) -> Vec<Vector2<usize>> {
        let adjacent = vec![
            Vector2::new(position.x, position.y - 1),
            Vector2::new(position.x - 1, position.y),
            Vector2::new(position.x + 1, position.y),
            Vector2::new(position.x, position.y + 1),
        ];

        adjacent
    }

    fn is_empty(&self, position: &Vector2<usize>) -> bool {
        if !self.is_inside(position) {
            return false;
        }

        if self.grid[position.y][position.x] == Tile::Wall {
            return false;
        }

        self.entities
            .iter()
            .filter(|e| e.is_alive())
            .find(|e| e.position == *position)
            .is_none()
    }

    fn is_inside(&self, position: &Vector2<usize>) -> bool {
        self.grid.len() > 0 && self.grid.len() > position.y && self.grid[0].len() > position.x
    }

    fn get_distances(&self, start: &Vector2<usize>) -> HashMap<Vector2<usize>, usize> {
        let mut distances = HashMap::new();
        distances.insert(*start, 0);

        let mut open_set = VecDeque::new();
        open_set.push_back(*start);
        let mut closed_set = HashSet::new();

        while let Some(p) = open_set.pop_front() {
            let successors = self
                .get_adjacent_tiles(&p)
                .into_iter()
                .filter(|c| self.is_empty(c));
            for s in successors {
                if closed_set.contains(&s) || open_set.contains(&s) {
                    continue;
                }

                open_set.push_back(s.clone());
                distances.insert(s, *distances.get(&p).unwrap() + 1);
            }

            closed_set.insert(p);
        }

        distances
    }

    fn attack(&mut self, attacker_index: usize, enemy_idx: usize) {
        self.entities[enemy_idx].hp -= self.entities[attacker_index].attack_power;
    }

    fn next_step(&self, entity_index: usize) -> Option<Vector2<usize>> {
        let me = &self.entities[entity_index];
        self.nearest_enemy(me)
            .and_then(|e| self.nearest_step(&me.position, &e))
    }

    fn nearest_step(&self, me: &Vector2<usize>, enemy: &Vector2<usize>) -> Option<Vector2<usize>> {
        let distances = self.get_distances(&enemy);
        self.get_adjacent_tiles(me)
            .into_iter()
            .filter(|p| self.is_empty(p))
            .filter_map(|p| distances.get(&p).map(|d| (p, d)))
            .min_by_key(|&(_, d)| d)
            .map(|(p, _)| p)
    }

    fn nearest_enemy(&self, me: &Entity) -> Option<Vector2<usize>> {
        let distances = self.get_distances(&me.position);
        self.get_all_enemies(me)
            .into_iter()
            .flat_map(|e| self.get_adjacent_tiles(&e))
            .filter(|p| self.is_empty(p))
            .filter_map(|p| distances.get(&p).map(|d| (p, d)))
            .min_by_key(|&(_, d)| d)
            .map(|(p, _)| p)
    }
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        Tile::Cavern => '.',
                        Tile::Wall => '#',
                    })
                    .collect()
            })
            .collect();

        for e in &self.entities {
            lines[e.position.y].replace_range(
                e.position.x..e.position.x + 1,
                match e.race {
                    Race::Goblin => "G",
                    Race::Elf => "E",
                },
            );
            lines[e.position.y].push_str(&format!(" {:?}({})", e.race, e.hp));
        }

        writeln!(f, "{}", lines.join("\n"))
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    position: Vector2<usize>,
    race: Race,
    hp: i32,
    attack_power: i32,
}

impl Entity {
    pub fn new(position: Vector2<usize>, race: Race) -> Entity {
        Entity {
            position,
            race,
            hp: 200,
            attack_power: 3,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn is_enemy(&self, other: &Entity) -> bool {
        self.race != other.race
    }
}

pub fn outcome(scene: &mut Scene) -> (i32, i32) {
    let mut round = 0;
    loop {
        let full_round = scene.tick();
        if !full_round {
            break;
        }

        round += 1;
    }

    let hp = scene.entities.iter().map(|e| e.hp).sum();
    (round, hp)
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Scene {
    let mut entities = vec![];
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Cavern,
                    'G' => {
                        entities.push(Entity::new(Vector2::new(x, y), Race::Goblin));
                        Tile::Cavern
                    }
                    'E' => {
                        entities.push(Entity::new(Vector2::new(x, y), Race::Elf));
                        Tile::Cavern
                    }
                    _ => unreachable!("Input has invalid character"),
                })
                .collect()
        })
        .collect();

    Scene { grid, entities }
}

#[aoc(day15, part1)]
fn solve_part1(scene: &Scene) -> i32 {
    let mut scene = scene.to_owned();
    let (round, hp) = outcome(&mut scene);
    round * hp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outcome1() {
        assert_eq!(
            outcome(&mut input_generator(
                r"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"
            )),
            (18, 1546)
        );
    }

    #[test]
    fn test_outcome2() {
        assert_eq!(
            outcome(&mut input_generator(
                r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
            )),
            (47, 590)
        );
    }

    #[test]
    fn test_outcome3() {
        assert_eq!(
            outcome(&mut input_generator(
                r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"
            )),
            (37, 982)
        );
    }

    #[test]
    fn test_outcome4() {
        assert_eq!(
            outcome(&mut input_generator(
                r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"
            )),
            (46, 859)
        );
    }
    #[test]
    fn test_outcome5() {
        assert_eq!(
            outcome(&mut input_generator(
                r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"
            )),
            (35, 793)
        );
    }
    #[test]
    fn test_outcome6() {
        assert_eq!(
            outcome(&mut input_generator(
                r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"
            )),
            (54, 536)
        );
    }
    #[test]
    fn test_outcome7() {
        assert_eq!(
            outcome(&mut input_generator(
                r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"
            )),
            (20, 937)
        );
    }
}
