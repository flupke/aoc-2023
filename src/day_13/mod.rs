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
    let previous_rows = iterator.collect::<Vec<_>>();
    (1..previous_rows.len()).find(|index| is_symetrical_with_smudge(&previous_rows, *index))
}

fn is_symetrical_with_smudge(rows: &Vec<&Vec<char>>, index: usize) -> bool {
    let mut left = index - 1;
    let mut right = index;
    let mut num_diffs = 0;
    loop {
        for (left_char, right_char) in rows[left].iter().zip(rows[right]) {
            if left_char != right_char {
                num_diffs += 1;
            }
            if num_diffs > 1 {
                return false;
            }
        }
        if left == 0 || right == rows.len() - 1 {
            break;
        }
        left -= 1;
        right += 1;
    }
    num_diffs == 1
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
