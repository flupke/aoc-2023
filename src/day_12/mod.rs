use std::usize;

use aoc_2023_rust_flupke::{split_numbers, Problem};
use memoize::memoize;

pub struct Day12;

struct Record {
    springs: String,
    damaged_chunks: Vec<usize>,
}

#[allow(dead_code)]
impl Record {
    fn from_str(input: &str) -> Self {
        let (springs, damaged_springs_str) = input.split_once(' ').unwrap();
        let damaged_chunks = split_numbers(damaged_springs_str, ',');
        Self {
            springs: springs.to_string(),
            damaged_chunks,
        }
    }

    fn expand(&self) -> Self {
        let springs = (0..5)
            .map(|_| self.springs.clone())
            .collect::<Vec<_>>()
            .join("?");
        let damaged_chunks = (0..5).flat_map(|_| &self.damaged_chunks).cloned().collect();
        Self {
            springs,
            damaged_chunks,
        }
    }

    fn arrangements(&self) -> usize {
        count_arrangements(self.springs.clone(), self.damaged_chunks.clone(), None)
    }
}

#[memoize]
fn count_arrangements(
    springs: String,
    chunk_lengths: Vec<usize>,
    current_chunk_length: Option<usize>,
) -> usize {
    if springs.is_empty() {
        return match (chunk_lengths.len(), current_chunk_length) {
            (0, None) => 1,
            (1, Some(current_chunk_length)) if chunk_lengths[0] == current_chunk_length => 1,
            _ => 0,
        };
    }
    if current_chunk_length.is_some() && chunk_lengths.is_empty() {
        return 0;
    }

    match (springs.as_bytes()[0], current_chunk_length) {
        (b'.', Some(current_chunk_length)) if current_chunk_length != chunk_lengths[0] => 0,
        (b'.', Some(_)) => {
            count_arrangements(springs[1..].to_string(), chunk_lengths[1..].to_vec(), None)
        }
        (b'.', None) => count_arrangements(springs[1..].to_string(), chunk_lengths, None),
        (b'#', Some(current_chunk_length)) => count_arrangements(
            springs[1..].to_string(),
            chunk_lengths,
            Some(current_chunk_length + 1),
        ),
        (b'#', None) => count_arrangements(springs[1..].to_string(), chunk_lengths, Some(1)),
        (b'?', Some(current_chunk_length)) => {
            let mut total = count_arrangements(
                springs[1..].to_string(),
                chunk_lengths.clone(),
                Some(current_chunk_length + 1),
            );
            if current_chunk_length == chunk_lengths[0] {
                total +=
                    count_arrangements(springs[1..].to_string(), chunk_lengths[1..].to_vec(), None);
            }
            total
        }
        (b'?', None) => {
            count_arrangements(springs[1..].to_string(), chunk_lengths.clone(), Some(1))
                + count_arrangements(springs[1..].to_string(), chunk_lengths, None)
        }
        _ => unreachable!(),
    }
}

struct RecordsList {
    records: Vec<Record>,
}

impl RecordsList {
    fn from_str(input: &str) -> Self {
        let records = input.lines().map(Record::from_str).collect();
        Self { records }
    }

    fn arrangements_sum(&self) -> usize {
        self.records
            .iter()
            .map(|record| record.expand().arrangements())
            .sum()
    }
}

impl Problem for Day12 {
    fn check(&self) {
        let records = RecordsList::from_str(include_str!("example.txt"));
        println!("number of arrangements: {}", records.arrangements_sum());
    }

    fn solve(&self) {
        let records = RecordsList::from_str(include_str!("input.txt"));
        println!("number of arrangements: {}", records.arrangements_sum());
    }
}
