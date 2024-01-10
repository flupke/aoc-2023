use std::usize;

use aoc_2023_rust_flupke::Problem;

pub struct Day15;

fn hash(input: &[u8]) -> u8 {
    let mut result: usize = 0;
    for char in input {
        result = (result + *char as usize) * 17 % 256;
    }
    result as u8
}

struct LensesMap {
    buckets: Vec<Vec<(String, u8)>>,
}

impl LensesMap {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 256],
        }
    }

    fn insert(&mut self, key: &str, value: u8) {
        let index = hash(key.as_bytes());
        let bucket = &mut self.buckets[index as usize];
        for entry in bucket.iter_mut() {
            if entry.0 == key {
                entry.1 = value;
                return;
            }
        }
        bucket.push((key.to_owned(), value));
    }

    fn remove(&mut self, key: &str) {
        let index = hash(key.as_bytes());
        let bucket = &mut self.buckets[index as usize];
        bucket.retain(|(k, _)| k != key);
    }

    fn score(&self) -> usize {
        self.buckets
            .iter()
            .enumerate()
            .flat_map(|(bucket_index, bucket)| {
                bucket
                    .iter()
                    .enumerate()
                    .map(move |(slot_index, (_, value))| {
                        (bucket_index + 1) * (slot_index + 1) * *value as usize
                    })
            })
            .sum()
    }
}

fn solve(input: &str) -> usize {
    let mut map = LensesMap::new();
    for entry in input.trim().split(',') {
        if entry.ends_with('-') {
            map.remove(entry.strip_suffix('-').unwrap());
        } else {
            let (key, value) = entry.split_once('=').unwrap();
            map.insert(key, value.parse::<u8>().unwrap());
        }
    }
    map.score()
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
