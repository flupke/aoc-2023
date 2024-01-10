use std::{collections::HashSet, usize};

use aoc_2023_rust_flupke::Problem;
use circular_buffer::CircularBuffer;
use itertools::Itertools;

use crate::common::{
    array::{parse, Array},
    vector::Vector,
};

pub struct Day14;

struct Platform {
    grid: Array<char>,
    coords: Vec<Vec<Vector>>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let grid = parse(input);
        let coords = vec![
            grid.iter_vec_coords().collect::<Vec<_>>(),       // North
            grid.iter_vec_col_coords().collect::<Vec<_>>(),   // West
            grid.iter_vec_coords().rev().collect::<Vec<_>>(), // South
            grid.iter_vec_col_coords().rev().collect::<Vec<_>>(), // East
        ];
        Self { grid, coords }
    }

    fn shake_until_stable(&mut self, cycles: usize) -> Vec<(usize, usize)> {
        let mut seen_beam_loads = HashSet::new();
        const STABLE_SEQUENCE_LENGTH: usize = 500;
        let mut last_lengths = CircularBuffer::<STABLE_SEQUENCE_LENGTH, usize>::new();
        let mut sequence_candidate = Vec::new();

        for i in 0..cycles {
            for direction in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                self.tilt(direction);
            }
            if last_lengths.len() == STABLE_SEQUENCE_LENGTH
                && last_lengths.iter().all_equal_value().is_ok()
            {
                sequence_candidate.push((i, self.north_beam_load()));
                if sequence_candidate.len() == STABLE_SEQUENCE_LENGTH {
                    break;
                }
            } else {
                seen_beam_loads.insert(self.north_beam_load());
                last_lengths.push_back(seen_beam_loads.len());
            }
        }

        sequence_candidate
    }

    fn tilt(&mut self, direction: Direction) {
        for coords in &self.coords[direction as usize] {
            if *self.grid.get(coords) == 'O' {
                move_rock(&mut self.grid, *coords, direction);
            }
        }
    }

    fn north_beam_load(&self) -> usize {
        let mut total_load = 0;
        for coords in self.grid.iter_coords() {
            if *self.grid.get(coords) == 'O' {
                total_load += self.grid.height - coords.1;
            }
        }
        total_load
    }
}

fn find_repeating_sequence(values: Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
    for sequence_length in 1..values.len() {
        let mut previous_chunk_loads = None;
        let mut last_chunk = None;
        let mut equal_count = 0;
        for chunk in values
            .chunks(sequence_length)
            .filter(|chunk| chunk.len() == sequence_length)
        {
            let chunk_loads = chunk.iter().map(|(_, load)| load).collect_vec();
            if previous_chunk_loads.is_none() {
                previous_chunk_loads = Some(chunk_loads);
            } else if previous_chunk_loads != Some(chunk_loads) {
                break;
            } else {
                equal_count += 1;
                last_chunk = Some(chunk);
            }
        }
        if equal_count > 1 {
            return last_chunk.map(|v| v.to_vec());
        }
    }
    None
}

#[derive(Clone, Copy)]
enum Direction {
    North = 0,
    West = 1,
    South = 2,
    East = 3,
}

impl Direction {
    fn vector(&self) -> Vector {
        match self {
            Self::North => Vector { x: 0, y: -1 },
            Self::South => Vector { x: 0, y: 1 },
            Self::West => Vector { x: -1, y: 0 },
            Self::East => Vector { x: 1, y: 0 },
        }
    }
}

fn move_rock(grid: &mut Array<char>, coords: Vector, direction: Direction) {
    let move_vector = direction.vector();
    let mut current_coords = coords;
    loop {
        current_coords += move_vector;
        if current_coords.x < 0
            || current_coords.x >= grid.width as i32
            || current_coords.y < 0
            || current_coords.y >= grid.height as i32
            || *grid.get(current_coords) != '.'
        {
            grid.set(coords, '.');
            grid.set(current_coords - move_vector, 'O');
            break;
        }
    }
}

fn solve(input: &str) -> usize {
    const CYCLES: usize = 1_000_000_000;
    let mut platform = Platform::parse(input);
    let loads = platform.shake_until_stable(CYCLES);
    let sequence = find_repeating_sequence(loads).unwrap();
    let index = (CYCLES - sequence.first().unwrap().0 - 1) % sequence.len();
    sequence[index].1
}

impl Problem for Day14 {
    fn check(&self) {
        println!("{}", solve(include_str!("example.txt")));
    }

    fn solve(&self) {
        println!("{}", solve(include_str!("input.txt")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilt_north() {
        let mut platform = Platform::parse(
            "
.O.
.#.
            "
            .trim(),
        );
        platform.tilt(Direction::North);
        assert_eq!(
            platform.grid.format(),
            "
.O.
.#.
            "
            .trim()
        );
    }

    #[test]
    fn test_tilt_west() {
        let mut platform = Platform::parse(
            "
.O.
.#.
            "
            .trim(),
        );
        platform.tilt(Direction::West);
        assert_eq!(
            platform.grid.format(),
            "
O..
.#.
            "
            .trim()
        );
    }

    #[test]
    fn test_tilt_south() {
        let mut platform = Platform::parse(
            "
.O.
.#.
            "
            .trim(),
        );
        platform.tilt(Direction::South);
        assert_eq!(
            platform.grid.format(),
            "
.O.
.#.
            "
            .trim()
        );
    }

    #[test]
    fn test_tilt_east() {
        let mut platform = Platform::parse(
            "
.O.
.#.
            "
            .trim(),
        );
        platform.tilt(Direction::East);
        assert_eq!(
            platform.grid.format(),
            "
..O
.#.
            "
            .trim()
        );
    }

    #[test]
    fn test_shake() {
        let mut platform = Platform::parse(include_str!("example.txt"));
        platform.shake_until_stable(1);
        assert_eq!(
            platform.grid.format(),
            "
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
            "
            .trim()
        );
        platform.shake_until_stable(1);
        assert_eq!(
            platform.grid.format(),
            "
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
            "
            .trim()
        );
        platform.shake_until_stable(1);
        assert_eq!(
            platform.grid.format(),
            "
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
            "
            .trim()
        );
    }
}
