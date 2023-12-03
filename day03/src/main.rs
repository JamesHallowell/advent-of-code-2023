use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

#[derive(Debug)]
struct Schematic {
    points: HashMap<Point, Value>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Value {
    Number(u32),
    Symbol(char),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Schematic {
    fn read_number(&self, point: Point) -> Option<(Point, u32)> {
        if !matches!(self.points.get(&point), Some(Value::Number(_))) {
            return None;
        }

        let start = successors(Some(point), |point| Some(point.left()))
            .find(|point| !matches!(self.points.get(point), Some(Value::Number(_))))?
            .right();

        let value = successors(Some(start), |point| Some(point.right()))
            .map_while(|point| match self.points.get(&point) {
                Some(Value::Number(number)) => Some(number),
                _ => None,
            })
            .fold(0, |acc, value| acc * 10 + value);

        Some((start, value))
    }
}

impl Point {
    fn adjacent(self) -> impl Iterator<Item = Point> {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .into_iter()
        .map(move |(x, y)| Point {
            x: self.x + x,
            y: self.y + y,
        })
    }

    fn left(self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}

fn parse(input: &'static str) -> Schematic {
    let points = input
        .lines()
        .enumerate()
        .fold(HashMap::default(), |mut points, (y, line)| {
            for (x, char) in line.chars().enumerate() {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };

                let value = match char {
                    '0'..='9' => char.to_digit(10).map(Value::Number),
                    '.' => None,
                    char => Some(Value::Symbol(char)),
                };

                if let Some(value) = value {
                    points.insert(point, value);
                }
            }

            points
        });

    Schematic { points }
}

fn part1(schematic: Schematic) -> u32 {
    let mut visited = HashSet::new();

    for (point, value) in &schematic.points {
        if let Value::Symbol(_) = value {
            for adjacent in point.adjacent() {
                if let Some(number) = schematic.read_number(adjacent) {
                    visited.insert(number);
                }
            }
        }
    }

    visited.into_iter().map(|(_, number)| number).sum()
}

fn part2(schematic: Schematic) -> u32 {
    let mut gear_ratios = 0;
    for (point, value) in &schematic.points {
        if let Value::Symbol('*') = value {
            let parts = point
                .adjacent()
                .filter_map(|point| schematic.read_number(point))
                .collect::<HashSet<_>>();

            if parts.len() == 2 {
                gear_ratios += parts.into_iter().map(|(_, number)| number).product::<u32>();
            }
        }
    }

    gear_ratios
}

fn main() {
    println!("Part 1: {}", part1(parse(include_str!("input.txt"))));
    println!("Part 2: {}", part2(parse(include_str!("input.txt"))));
}
