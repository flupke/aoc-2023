use std::{collections::HashMap, ops::Range};

use aoc_2023_rust_flupke::{split_numbers, Problem};

pub struct Day5;

#[derive(Debug)]
struct CorrespondenceRange {
    source: Range<u64>,
    destination: u64,
}

impl CorrespondenceRange {
    fn new(source: u64, destination: u64, range: u64) -> Self {
        Self {
            source: source..(source + range),
            destination,
        }
    }

    fn map(&self, value: u64) -> Option<u64> {
        if self.source.contains(&value) {
            Some(self.destination + value - self.source.start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct CorrespondenceMap {
    source: String,
    destination: String,
    ranges: Vec<CorrespondenceRange>,
}

impl CorrespondenceMap {
    fn new(source: String, destination: String) -> Self {
        Self {
            source,
            destination,
            ranges: Vec::new(),
        }
    }

    fn get(&self, value: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| range.map(value))
            .unwrap_or(value)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    /// Correspondences maps, indexed by source
    correspondences: HashMap<String, CorrespondenceMap>,
}

impl Almanac {
    fn new(seeds: Vec<u64>) -> Self {
        Self {
            seeds,
            correspondences: HashMap::new(),
        }
    }

    fn from_text(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds_line = lines.next().unwrap();
        let seeds = split_numbers(seeds_line.split_once(":").unwrap().1);
        let mut almanac = Self::new(seeds);
        let mut current_correspondence: Option<CorrespondenceMap> = None;

        for mut line in lines {
            line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.ends_with(":") {
                store_correspondence(&mut current_correspondence, &mut almanac.correspondences);
                let (source, destination) = parse_correspondence_title(line);
                current_correspondence = Some(CorrespondenceMap::new(source, destination));
            } else {
                let numbers = split_numbers(line);
                let destination = numbers[0];
                let source = numbers[1];
                let range = numbers[2];
                if let Some(current_correspondence) = &mut current_correspondence {
                    current_correspondence.ranges.push(CorrespondenceRange::new(
                        source,
                        destination,
                        range,
                    ));
                }
            }
        }
        store_correspondence(&mut current_correspondence, &mut almanac.correspondences);

        almanac
    }

    fn search(&self, source: &str, source_value: u64, destination: &str) -> u64 {
        let mut current_source = source;
        let mut current_value = source_value;
        while current_source != destination {
            let map = &self.correspondences[current_source];
            current_value = map.get(current_value);
            current_source = &map.destination;
        }
        current_value
    }

    fn find_lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.search("seed", *seed, "location"))
            .min()
            .unwrap()
    }
}

fn store_correspondence(
    current_correspondence: &mut Option<CorrespondenceMap>,
    correspondences: &mut HashMap<String, CorrespondenceMap>,
) {
    if let Some(correspondence) = current_correspondence.take() {
        correspondences.insert(correspondence.source.clone(), correspondence);
    }
}

fn parse_correspondence_title(line: &str) -> (String, String) {
    let line = line.trim_end_matches(" map:");
    let (source, destination) = line.split_once("-to-").unwrap();
    (source.to_string(), destination.to_string())
}

impl Problem for Day5 {
    fn check(&self) {
        let almanac =
            Almanac::from_text(&std::fs::read_to_string("src/day_5/example.txt").unwrap());
        println!("min location: {}", almanac.find_lowest_location());
    }

    fn solve(&self) {
        let almanac = Almanac::from_text(&std::fs::read_to_string("src/day_5/input.txt").unwrap());
        println!("min location: {}", almanac.find_lowest_location());
    }
}
