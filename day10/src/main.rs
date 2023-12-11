use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum TileType {
    Vertical,
    Horizontal,
    NorthToEastBend,
    NorthToWestBend,
    SouthToWestBend,
    SouthToEastBend,
    Start,
    Ground,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Tile {
    tile_type: TileType,
    position: Position,
}

#[derive(Debug, Clone)]
struct Sketch {
    tiles: Vec<Tile>,
}

fn parse_tile(c: char) -> TileType {
    match c {
        '|' => TileType::Vertical,
        '-' => TileType::Horizontal,
        'L' => TileType::NorthToEastBend,
        'J' => TileType::NorthToWestBend,
        '7' => TileType::SouthToWestBend,
        'F' => TileType::SouthToEastBend,
        'S' => TileType::Start,
        '.' => TileType::Ground,
        _ => panic!("Invalid tile"),
    }
}

fn parse_tiles(input: &'static str) -> Sketch {
    let tiles = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| Tile {
                tile_type: parse_tile(char),
                position: Position {
                    x: x as i32,
                    y: y as i32,
                },
            })
        })
        .collect();

    Sketch { tiles }
}

impl Position {
    fn up(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl Tile {
    fn connections(&self) -> impl Iterator<Item = Position> {
        match self.tile_type {
            TileType::Vertical => [
                Some(self.position.up()),
                Some(self.position.down()),
                None,
                None,
            ],
            TileType::Horizontal => [
                Some(self.position.left()),
                Some(self.position.right()),
                None,
                None,
            ],
            TileType::NorthToEastBend => [
                Some(self.position.up()),
                Some(self.position.right()),
                None,
                None,
            ],
            TileType::NorthToWestBend => [
                Some(self.position.up()),
                Some(self.position.left()),
                None,
                None,
            ],
            TileType::SouthToWestBend => [
                Some(self.position.down()),
                Some(self.position.left()),
                None,
                None,
            ],
            TileType::SouthToEastBend => [
                Some(self.position.down()),
                Some(self.position.right()),
                None,
                None,
            ],
            TileType::Start => [
                Some(self.position.up()),
                Some(self.position.down()),
                Some(self.position.left()),
                Some(self.position.right()),
            ],
            TileType::Ground => [None, None, None, None],
        }
        .into_iter()
        .flatten()
    }
}

impl Sketch {
    fn start(&self) -> Tile {
        self.tiles
            .iter()
            .find(|tile| tile.tile_type == TileType::Start)
            .copied()
            .unwrap()
    }

    fn get(&self, pos: Position) -> Option<Tile> {
        self.tiles.iter().find(|tile| tile.position == pos).copied()
    }

    fn is_connected(&self, src: Tile, dst: Tile) -> bool {
        src.connections().any(|pos| pos == dst.position)
            && dst.connections().any(|pos| pos == src.position)
    }

    fn connections(&self, tile: Tile) -> impl Iterator<Item = Tile> + '_ {
        tile.connections().filter_map(|pos| self.get(pos))
    }

    fn tiles_in_pipe_loop(&self) -> HashSet<Tile> {
        let start = self.start();
        let mut to_visit = self
            .connections(start)
            .filter(|&tile| self.is_connected(start, tile))
            .collect::<VecDeque<_>>();
        assert_eq!(to_visit.len(), 2);

        let mut visited = HashSet::from([start]);

        while let Some(current) = to_visit.pop_front() {
            visited.insert(current);
            for next in self
                .connections(current)
                .filter(|tile| !visited.contains(tile) && self.is_connected(current, *tile))
            {
                to_visit.push_back(next);
            }
        }

        visited
    }
}

fn distances_from_start(pipes: &HashSet<Tile>) -> usize {
    pipes.len() / 2
}

fn area_enclosed_by_loop(sketch: &Sketch, pipes: &HashSet<Tile>) -> i32 {
    let mut inside = false;
    let mut count = 0;

    for tile in &sketch.tiles {
        if tile.position.x == 0 {
            inside = false;
        }

        if pipes.contains(tile) {
            if let Some(up) = sketch.get(tile.position.up()) {
                if sketch.is_connected(*tile, up) {
                    inside = !inside;
                }
            }
        } else if inside {
            count += 1;
        }
    }

    count
}

fn main() {
    let sketch = parse_tiles(include_str!("input.txt"));
    let pipe_tiles = sketch.tiles_in_pipe_loop();

    println!("Part 1: {:?}", distances_from_start(&pipe_tiles));
    println!("Part 2: {:?}", area_enclosed_by_loop(&sketch, &pipe_tiles));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &'static str = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        let sketch = parse_tiles(INPUT);
        assert_eq!(distances_from_start(&sketch.tiles_in_pipe_loop()), 4);
    }

    #[test]
    fn complex_example() {
        const INPUT: &'static str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        let sketch = parse_tiles(INPUT);
        assert_eq!(distances_from_start(&sketch.tiles_in_pipe_loop()), 8);
    }

    #[test]
    fn enclosed() {
        const INPUT: &'static str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

        let sketch = parse_tiles(INPUT);
        assert_eq!(
            area_enclosed_by_loop(&sketch, &sketch.tiles_in_pipe_loop()),
            4
        );
    }
}
