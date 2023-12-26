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

    fn expand(&self, rate: usize) -> Vec<Vector> {
        let empty_rows = empty_indices(self.map.rows());
        let empty_columns = empty_indices(self.map.columns().iter());

        let mut expanded_galaxies = Vec::new();
        let mut y_shift = 0;
        for y in 0..self.map.height() {
            let mut x_shift = 0;
            for x in 0..self.map.width() {
                if let Some(Tile::Galaxy) = self.map.get(&Vector {
                    x: x as i32,
                    y: y as i32,
                }) {
                    expanded_galaxies.push(Vector {
                        x: x as i32 + x_shift as i32,
                        y: y as i32 + y_shift as i32,
                    });
                }
                if empty_columns.contains(&x) {
                    x_shift += rate - 1;
                }
            }
            if empty_rows.contains(&y) {
                y_shift += rate - 1;
            }
        }

        expanded_galaxies
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

fn pairs_manhattan_distance_sum(points: &[Vector]) -> usize {
    points
        .iter()
        .combinations(2)
        .map(|chunk| chunk[0].manhattan_distance(chunk[1]) as usize)
        .sum()
}

impl Problem for Day11 {
    fn check(&self) {
        let universe = Universe::parse(include_str!("example.txt"));
        println!("{}", pairs_manhattan_distance_sum(&universe.expand(100)));
    }

    fn solve(&self) {
        let universe = Universe::parse(include_str!("input.txt"));
        println!(
            "{}",
            pairs_manhattan_distance_sum(&universe.expand(1000000))
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        let universe = Universe::parse(
            "
#.#
...
#.#
"
            .trim(),
        );
        assert_eq!(
            universe.expand(1),
            vec![
                Vector { x: 0, y: 0 },
                Vector { x: 2, y: 0 },
                Vector { x: 0, y: 2 },
                Vector { x: 2, y: 2 },
            ]
        );
        assert_eq!(
            universe.expand(2),
            vec![
                Vector { x: 0, y: 0 },
                Vector { x: 3, y: 0 },
                Vector { x: 0, y: 3 },
                Vector { x: 3, y: 3 },
            ]
        );
    }
}
