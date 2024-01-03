use std::usize;

use aoc_2023_rust_flupke::Problem;

use crate::common::array;

pub struct Day13;

type Pattern = array::Array<char>;

impl Pattern {
    fn score(&self) -> usize {
        let mut score = 0;
        score += search_reflection(&mut self.rows()).unwrap_or(0) * 100;
        score += search_reflection(&mut self.columns().iter()).unwrap_or(0);
        score
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(array::parse).collect()
}

fn search_reflection(iterator: &mut dyn Iterator<Item = &Vec<char>>) -> Option<usize> {
    let mut previous_rows = Vec::new();
    let mut candidates = Vec::new();
    for (position, row) in iterator.enumerate() {
        if let Some(previous_row) = previous_rows.last() {
            if row == *previous_row {
                candidates.push(position);
            }
        }
        previous_rows.push(row);
    }

    candidates
        .iter()
        .find(|index| is_symetrical(&previous_rows, **index))
        .cloned()
}

fn is_symetrical(rows: &Vec<&Vec<char>>, index: usize) -> bool {
    let mut left = index - 1;
    let mut right = index;
    loop {
        if rows[left] != rows[right] {
            return false;
        }
        if left == 0 || right == rows.len() - 1 {
            break;
        }
        left -= 1;
        right += 1;
    }
    true
}

fn solve(input: &str) -> usize {
    let patterns = parse(input);
    patterns.iter().map(|pattern| pattern.score()).sum()
}

impl Problem for Day13 {
    fn check(&self) {
        println!("score: {}", solve(include_str!("example.txt")));
    }
    fn solve(&self) {
        println!("score: {}", solve(include_str!("input.txt")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_reflection() {
        let array = array::parse(
            "
..####..####.##.#
###..###..#######
..####....###..##
...##...###..##..
..........###..##
##.##.####...##..
.#.......#.......
#......##.#######
.##..##.#.#.####.
"
            .trim(),
        );

        assert_eq!(search_reflection(&mut array.columns().iter()), Some(14));
    }
}
