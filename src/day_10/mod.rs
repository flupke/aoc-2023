mod array;
mod vector;

use std::fmt::Display;

use aoc_2023_rust_flupke::Problem;

use self::{
    array::{Array, ArrayManipulator},
    vector::Vector,
};

pub struct Day10;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Start,
    HorizontalPipe,
    VerticalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
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

struct Map {
    tiles: Array<Tile>,
}

impl ArrayManipulator<Tile> for Map {
    fn data_mut(&mut self) -> &mut Array<Tile> {
        &mut self.tiles
    }

    fn data(&self) -> &Array<Tile> {
        &self.tiles
    }
}

impl Map {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect::<Array<Tile>>();

        Self { tiles }
    }

    fn find_start(&self) -> Vector {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.tiles[y][x] == Tile::Start {
                    return Vector {
                        x: x as i32,
                        y: y as i32,
                    };
                }
            }
        }
        panic!("no start found");
    }
}

struct MapDistanceCounter {
    counts: Array<usize>,
}

impl ArrayManipulator<usize> for MapDistanceCounter {
    fn data_mut(&mut self) -> &mut Array<usize> {
        &mut self.counts
    }

    fn data(&self) -> &Array<usize> {
        &self.counts
    }
}

impl MapDistanceCounter {
    fn new(dimensions: &Vector) -> Self {
        let counts = vec![vec![0; dimensions.x as usize]; dimensions.y as usize];
        Self { counts }
    }

    fn max_distance(&self) -> usize {
        *self
            .counts
            .iter()
            .map(|line| line.iter().max().unwrap_or(&0))
            .max()
            .unwrap_or(&0)
    }

    fn min(&self, other: &MapDistanceCounter) -> MapDistanceCounter {
        MapDistanceCounter {
            counts: self
                .data()
                .iter()
                .zip(other.data())
                .map(|(a, b)| {
                    a.iter()
                        .zip(b)
                        .map(|(a, b)| if a < b { *a } else { *b })
                        .collect()
                })
                .collect(),
        }
    }
}

fn get_max_distance(map: &Map) -> usize {
    let start_position = map.find_start();

    // Find the directions that lead to pipes entries from the starting position.
    let mut start_directions = Vec::new();
    for neighbor_delta in neighbors_deltas() {
        let neighbor_position = start_position.add(&neighbor_delta);
        let tile = map.get(&neighbor_position);
        if tile.is_some() && tile.unwrap().is_pipe() {
            for pipe_deltas in tile.unwrap().connected_tiles() {
                if pipe_deltas == neighbor_delta.neg() {
                    start_directions.push(neighbor_delta.clone());
                }
            }
        }
    }

    // For each starting direction, follow the pipe and count the distance.
    start_directions
        .iter()
        .map(|direction| walk_map(map, &start_position, direction))
        .reduce(|a, b| a.min(&b))
        .unwrap()
        .max_distance()
}

fn walk_map(map: &Map, start_position: &Vector, start_direction: &Vector) -> MapDistanceCounter {
    let mut position = start_position.clone();
    let mut direction = start_direction.clone();
    let mut distance = 0;
    let mut counter = MapDistanceCounter::new(&map.dimensions());
    loop {
        position = position.add(&direction);
        let tile = map.get(&position).unwrap();
        if tile == Tile::Start {
            break;
        } else {
            distance += 1;
            counter.set(&position, distance);
            direction = tile.move_through(&direction);
        }
    }
    counter
}

impl Problem for Day10 {
    fn check(&self) {
        let map = Map::parse(include_str!("example.txt"));
        println!("max distance: {}", get_max_distance(&map));
    }
    fn solve(&self) {
        let map = Map::parse(include_str!("input.txt"));
        println!("max distance: {}", get_max_distance(&map));
    }
}
