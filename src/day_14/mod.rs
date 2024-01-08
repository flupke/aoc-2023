use aoc_2023_rust_flupke::Problem;

use crate::common::array::{parse, Array, ArrayCoordinates};

pub struct Day14;

struct Platform {
    grid: Array<char>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        Self { grid: parse(input) }
    }

    fn tilt(&mut self) {
        for coords in self.grid.iter_coords() {
            if *self.grid.get(coords) == 'O' {
                move_to_top(&mut self.grid, coords);
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

fn move_to_top(grid: &mut Array<char>, coords: ArrayCoordinates) {
    if coords.1 == 0 {
        return;
    }
    let mut prev_char = 'O';
    for y in (0..coords.1).rev() {
        let char = *grid.get((coords.0, y));
        if y == 0 && char == '.' {
            grid.set(coords, '.');
            grid.set((coords.0, 0), 'O');
            break;
        } else if char != '.' {
            if prev_char != 'O' {
                grid.set(coords, '.');
                grid.set((coords.0, y + 1), 'O');
            }
            break;
        }
        prev_char = char;
    }
}

fn solve(input: &str) -> usize {
    let mut platform = Platform::parse(input);
    platform.tilt();
    platform.north_beam_load()
}

impl Problem for Day14 {
    fn check(&self) {
        println!("{}", solve(include_str!("example.txt")));
    }

    fn solve(&self) {
        println!("{}", solve(include_str!("input.txt")));
    }
}
