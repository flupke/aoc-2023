use std::{collections::HashSet, fmt::Display, i32, usize};

use aoc_2023_rust_flupke::Problem;

use super::common::{array::Array, vector::Vector};

pub struct Day10;

#[derive(PartialEq, Debug, Clone, Copy, Default)]
enum Tile {
    Start,
    HorizontalPipe,
    VerticalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    #[default]
    Ground,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "S"),
            Self::HorizontalPipe => write!(f, "-"),
            Self::VerticalPipe => write!(f, "|"),
            Self::NorthEastPipe => write!(f, "L"),
            Self::NorthWestPipe => write!(f, "J"),
            Self::SouthWestPipe => write!(f, "7"),
            Self::SouthEastPipe => write!(f, "F"),
            Self::Ground => write!(f, "."),
        }
    }
}

impl Tile {
    fn from_char(input: char) -> Self {
        match input {
            'S' => Self::Start,
            '-' => Self::HorizontalPipe,
            '|' => Self::VerticalPipe,
            'L' => Self::NorthEastPipe,
            'J' => Self::NorthWestPipe,
            '7' => Self::SouthWestPipe,
            'F' => Self::SouthEastPipe,
            '.' => Self::Ground,
            _ => panic!("invalid tile: {:?}", input),
        }
    }

    fn connected_tiles(&self) -> impl Iterator<Item = Vector> {
        match self {
            Self::HorizontalPipe => {
                IntoIterator::into_iter([Vector { x: -1, y: 0 }, Vector { x: 1, y: 0 }])
            }
            Self::VerticalPipe => {
                IntoIterator::into_iter([Vector { x: 0, y: 1 }, Vector { x: 0, y: -1 }])
            }
            Self::NorthEastPipe => {
                IntoIterator::into_iter([Vector { x: 0, y: -1 }, Vector { x: 1, y: 0 }])
            }
            Self::NorthWestPipe => {
                IntoIterator::into_iter([Vector { x: 0, y: -1 }, Vector { x: -1, y: 0 }])
            }
            Self::SouthWestPipe => {
                IntoIterator::into_iter([Vector { x: 0, y: 1 }, Vector { x: -1, y: 0 }])
            }
            Self::SouthEastPipe => {
                IntoIterator::into_iter([Vector { x: 0, y: 1 }, Vector { x: 1, y: 0 }])
            }
            _ => panic!("not a pipe: {:?}", self),
        }
    }

    fn move_through(&self, direction: &Vector) -> Vector {
        match (self, direction) {
            (Self::HorizontalPipe, Vector { x: 1 | -1, y: 0 }) => direction.clone(),
            (Self::VerticalPipe, Vector { x: 0, y: 1 | -1 }) => direction.clone(),
            (Self::NorthEastPipe, Vector { x: 0, y: 1 }) => Vector { x: 1, y: 0 },
            (Self::NorthEastPipe, Vector { x: -1, y: 0 }) => Vector { x: 0, y: -1 },
            (Self::NorthWestPipe, Vector { x: 0, y: 1 }) => Vector { x: -1, y: 0 },
            (Self::NorthWestPipe, Vector { x: 1, y: 0 }) => Vector { x: 0, y: -1 },
            (Self::SouthWestPipe, Vector { x: 0, y: -1 }) => Vector { x: -1, y: 0 },
            (Self::SouthWestPipe, Vector { x: 1, y: 0 }) => Vector { x: 0, y: 1 },
            (Self::SouthEastPipe, Vector { x: 0, y: -1 }) => Vector { x: 1, y: 0 },
            (Self::SouthEastPipe, Vector { x: -1, y: 0 }) => Vector { x: 0, y: 1 },
            _ => panic!("cannot move through: {:?} {:?}", self, direction),
        }
    }

    fn is_pipe(&self) -> bool {
        matches!(
            self,
            Self::HorizontalPipe
                | Self::VerticalPipe
                | Self::NorthEastPipe
                | Self::NorthWestPipe
                | Self::SouthWestPipe
                | Self::SouthEastPipe
        )
    }
}

fn neighbors_deltas() -> impl Iterator<Item = Vector> {
    IntoIterator::into_iter([
        Vector { x: -1, y: 0 },
        Vector { x: 1, y: 0 },
        Vector { x: 0, y: -1 },
        Vector { x: 0, y: 1 },
    ])
}

#[derive(Clone)]
struct Map {
    tiles: Array<Tile>,
    start_tile: Tile,
}

impl Map {
    fn parse(input: &str, start_tile: Tile) -> Self {
        let tiles = input
            .lines()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect::<Array<Tile>>();

        Self { tiles, start_tile }
    }

    fn find_start(&self) -> Vector {
        for y in 0..self.tiles.height {
            for x in 0..self.tiles.width {
                if self.tiles.get(&Vector {
                    x: x as i32,
                    y: y as i32,
                }) == Tile::Start
                {
                    return Vector {
                        x: x as i32,
                        y: y as i32,
                    };
                }
            }
        }
        panic!("no start found");
    }

    fn enclosed_area(&self) -> usize {
        let loop_info = LoopInfo::new(self);
        let mut area = 0;
        for y in 0..self.tiles.height {
            let mut crosses_sum = 0;
            for x in 0..self.tiles.width {
                let position = Vector {
                    x: x as i32,
                    y: y as i32,
                };
                crosses_sum += loop_info.cross_directions.get(&position);
                if !loop_info.tiles.contains(&position) && crosses_sum != 0 {
                    area += 1;
                }
            }
        }
        area
    }
}

struct LoopInfo {
    /// A set containing the visited tiles.
    tiles: HashSet<Vector>,

    /// An integer representing the crosss "direction" of a ray going to the left of the map
    /// through this tile. It is equal to 1 if the pipe was walked going south in this tile, -1
    /// going north, and 0 otherwise. This alows to implements Dan's Sunday winding number
    /// algorithm efficiently.
    cross_directions: Array<i32>,
}

impl LoopInfo {
    fn new(map: &Map) -> Self {
        let start_position = map.find_start();
        let mut start_direction = None;
        for neighbor_delta in neighbors_deltas() {
            let neighbor_position = start_position.add(&neighbor_delta);
            let tile = map.tiles.get(&neighbor_position);
            if tile.is_pipe() {
                for pipe_deltas in tile.connected_tiles() {
                    if pipe_deltas == neighbor_delta.neg() {
                        start_direction = Some(neighbor_delta.clone());
                        break;
                    }
                }
            }
        }

        let mut position = start_position.clone();
        let mut direction = start_direction.unwrap().clone();
        let mut cross_direction = vec![vec![0; map.tiles.width]; map.tiles.height];
        let mut tiles = HashSet::new();
        let mut map = map.clone();
        map.tiles.set(&start_position, map.start_tile);
        loop {
            tiles.insert(position.clone());
            position = position.add(&direction);
            let tile = map.tiles.get(&position);
            let in_y = direction.y;
            direction = tile.move_through(&direction);
            let out_y = direction.y;
            if tile == Tile::SouthEastPipe
                || tile == Tile::SouthWestPipe
                || tile == Tile::VerticalPipe
            {
                cross_direction[position.y as usize][position.x as usize] =
                    if in_y != 0 { in_y } else { out_y };
            }
            if position == start_position {
                break;
            }
        }

        Self {
            cross_directions: Array::from_iter(cross_direction),
            tiles,
        }
    }
}

impl Problem for Day10 {
    fn check(&self) {
        let map = Map::parse(include_str!("example.txt"), Tile::from_char('|'));
        println!("enclosed area: {}", map.enclosed_area());
    }
    fn solve(&self) {
        let map = Map::parse(include_str!("input.txt"), Tile::from_char('-'));
        println!("enclosed area: {}", map.enclosed_area());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_enclosed_area_of_complex_loop() {
        let map = Map::parse(include_str!("example.txt"), Tile::from_char('|'));
        assert_eq!(map.enclosed_area(), 9);
    }

    #[test]
    fn test_get_enclosed_area_of_simple_loop() {
        let map = Map::parse(include_str!("simple_loop.txt"), Tile::from_char('F'));
        assert_eq!(map.enclosed_area(), 1);
    }

    #[test]
    fn test_loop_info_on_simple_loop() {
        let map = Map::parse(include_str!("simple_loop.txt"), Tile::from_char('F'));
        let loop_info = LoopInfo::new(&map);
        assert_eq!(
            loop_info.cross_directions.shift(&1).format(),
            "
11111
12101
12101
11111
11111
        "
            .trim()
        );
    }

    #[test]
    fn test_loop_info_on_complex_loop() {
        let map = Map::parse(include_str!("example.txt"), Tile::from_char('|'));
        let loop_info = LoopInfo::new(&map);
        assert_eq!(
            loop_info.cross_directions.shift(&1).format(),
            "
111111111111
121111111101
120111112101
120111112101
120111112101
121101211101
121101211101
111111111111
111111111111
"
            .trim()
        );
    }
}
