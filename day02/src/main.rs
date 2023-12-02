use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    game: &'static str,
}

#[derive(Debug)]
struct Set {
    set: &'static str,
}

#[derive(Debug, Default)]
struct Bag {
    cubes: HashMap<Cube, usize>,
}

impl FromIterator<(Cube, usize)> for Bag {
    fn from_iter<T: IntoIterator<Item = (Cube, usize)>>(iter: T) -> Self {
        Self {
            cubes: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Cube {
    Red,
    Green,
    Blue,
}

fn games(input: &'static str) -> impl Iterator<Item = Game> {
    input.lines().map(|game| Game { game })
}

impl Game {
    fn number(&self) -> usize {
        let (game, _) = self.game.split_once(':').unwrap();
        let (_, game_id) = game.split_once(' ').unwrap();
        game_id.parse().unwrap()
    }

    fn sets(&self) -> impl Iterator<Item = Set> {
        let (_, sets) = self.game.split_once(':').unwrap();
        sets.split(';').map(|set| Set { set: set.trim() })
    }

    fn minimum_bag(&self) -> Bag {
        self.sets().fold(Bag::default(), |mut bag, set| {
            for (cube, count) in set.reveals() {
                let current_count = bag.cubes.entry(cube).or_insert(0);
                if count > *current_count {
                    *current_count = count;
                }
            }
            bag
        })
    }
}

impl Set {
    fn reveals(&self) -> impl Iterator<Item = (Cube, usize)> {
        self.set.split(',').map(|count| {
            let (count, color) = count.trim().split_once(' ').unwrap();

            let color = match color {
                "red" => Cube::Red,
                "green" => Cube::Green,
                "blue" => Cube::Blue,
                _ => panic!("unexpected color"),
            };
            let count = count.parse().unwrap();

            (color, count)
        })
    }
}

impl Bag {
    fn count(&self, cube: Cube) -> usize {
        self.cubes.get(&cube).copied().unwrap_or_default()
    }

    fn power(&self) -> usize {
        [Cube::Red, Cube::Green, Cube::Blue]
            .into_iter()
            .map(|cube| self.count(cube))
            .product()
    }

    fn is_subset_of(&self, other: &Self) -> bool {
        [Cube::Red, Cube::Green, Cube::Blue]
            .into_iter()
            .all(|cube| self.count(cube) <= other.count(cube))
    }
}

fn possible_games(games: impl Iterator<Item = Game>, bag: Bag) -> impl Iterator<Item = Game> {
    games.filter(move |game| game.minimum_bag().is_subset_of(&bag))
}

fn part1(games: impl Iterator<Item = Game>) -> usize {
    let bag = [(Cube::Red, 12), (Cube::Green, 13), (Cube::Blue, 14)]
        .into_iter()
        .collect();

    possible_games(games, bag).map(|game| game.number()).sum()
}

fn part2(games: impl Iterator<Item = Game>) -> usize {
    games
        .map(|game| game.minimum_bag())
        .map(|bag| bag.power())
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(games(include_str!("input.txt"))));
    println!("Part 2: {}", part2(games(include_str!("input.txt"))));
}
