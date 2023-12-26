use aoc_2023_rust_flupke::Problem;

use itertools::Itertools;

use crate::common::{array::Array, vector::Vector};
use std::{fmt::Display, usize};

pub struct Day11;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Void,
    Galaxy,
}

impl Tile {
    fn from_char(input: char) -> Self {
        match input {
            '.' => Self::Void,
            '#' => Self::Galaxy,
            _ => panic!("invalid tile: {:?}", input),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "."),
            Self::Galaxy => write!(f, "#"),
        }
    }
}

struct Universe {
    map: Array<Tile>,
}

impl Universe {
    fn parse(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .map(|line| line.chars().map(Tile::from_char).collect())
                .collect(),
        }
    }

    fn expand(&self) -> Self {
        let empty_rows = empty_indices(self.map.rows());
        let empty_columns = empty_indices(self.map.columns().iter());

        let mut data = Vec::new();
        for y in 0..self.map.height() {
            let mut row = Vec::new();
            if empty_rows.contains(&y) {
                data.push(vec![Tile::Void; self.map.width() + empty_columns.len()]);
                data.push(vec![Tile::Void; self.map.width() + empty_columns.len()]);
            } else {
                for x in 0..self.map.width() {
                    if empty_columns.contains(&x) {
                        row.push(Tile::Void);
                        row.push(Tile::Void);
                    } else {
                        row.push(
                            self.map
                                .get(&Vector {
                                    x: x as i32,
                                    y: y as i32,
                                })
                                .unwrap(),
                        );
                    }
                }
                data.push(row);
            }
        }

        Self {
            map: Array::new(data),
        }
    }

    fn galaxies_pairs_manhattan_distance_sum(&self) -> i32 {
        self.map
            .into_iter()
            .filter_map(|(position, tile)| {
                if *tile == Tile::Galaxy {
                    Some(position)
                } else {
                    None
                }
            })
            .combinations(2)
            .map(|chunk| chunk[0].manhattan_distance(&chunk[1]))
            .sum()
    }
}

fn empty_indices<'a>(iterator: impl Iterator<Item = &'a Vec<Tile>>) -> Vec<usize> {
    iterator
        .enumerate()
        .filter_map(|(index, row)| {
            if row.iter().all(|tile| *tile == Tile::Void) {
                Some(index)
            } else {
                None
            }
        })
        .collect()
}

impl Problem for Day11 {
    fn check(&self) {
        let universe = Universe::parse(include_str!("example.txt"));
        println!(
            "{}",
            universe.expand().galaxies_pairs_manhattan_distance_sum()
        );
    }

    fn solve(&self) {
        let universe = Universe::parse(include_str!("input.txt"));
        println!(
            "{}",
            universe.expand().galaxies_pairs_manhattan_distance_sum()
        );
    }
}
