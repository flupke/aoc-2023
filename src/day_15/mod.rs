use std::usize;

use aoc_2023_rust_flupke::Problem;

pub struct Day15;

fn hash(input: &[u8]) -> usize {
    let mut result: usize = 0;
    for char in input {
        result = (result + *char as usize) * 17 % 256;
    }
    result
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|word| hash(word.as_bytes()))
        .sum()
}

impl Problem for Day15 {
    fn solve(&self) {
        println!("{}", solve(include_str!("input.txt")));
    }

    fn check(&self) {
        println!("{}", solve(include_str!("example.txt")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH".as_bytes()), 52);
    }
}
