#![allow(clippy::single_range_in_vec_init)]

use std::{collections::HashMap, ops::Range};

use aoc_2023_rust_flupke::{split_numbers, Problem};

pub struct Day5;

#[derive(Debug)]
struct Mapper {
    source: Range<u64>,
    destination: u64,
}

impl Mapper {
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

    /// Cut a range with this mapper's range.
    fn map_range(&self, range: &Range<u64>) -> Option<Vec<Range<u64>>> {
        let mut ranges = Vec::new();
        if (range.start >= self.source.end) || (range.end <= self.source.start) {
            // Range out of mapper
            return None;
        } else if self.source.contains(&range.start) && self.source.contains(&(range.end - 1)) {
            // Mapper contains range
            ranges.push(self.map(range.start).unwrap()..self.map(range.end - 1).unwrap() + 1);
        } else if self.source.contains(&range.start) {
            // Range start is in the mapper
            ranges.push(self.map(range.start).unwrap()..self.map(self.source.end - 1).unwrap() + 1);
            ranges.push(self.source.end..range.end);
        } else if self.source.contains(&(range.end - 1)) {
            // Range end is in the mapper
            ranges.push(range.start..self.source.start);
            ranges.push(self.map(self.source.start).unwrap()..self.map(range.end - 1).unwrap() + 1);
        } else {
            // Range contains the mapper
            ranges.push(range.start..self.source.start);
            ranges.push(
                self.map(self.source.start).unwrap()..self.map(self.source.end - 1).unwrap() + 1,
            );
            ranges.push(self.source.end..range.end)
        }
        Some(ranges)
    }
}

#[derive(Debug)]
struct CorrespondenceMap {
    source: String,
    destination: String,
    mappers: Vec<Mapper>,
}

impl CorrespondenceMap {
    fn new(source: &str, destination: &str) -> Self {
        Self {
            source: source.to_string(),
            destination: destination.to_string(),
            mappers: Vec::new(),
        }
    }

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        self.mappers
            .iter()
            .find_map(|mapper| mapper.map_range(range))
            .unwrap_or(vec![range.clone()])
    }
}

#[derive(Debug)]
struct Almanac {
    seeds_ranges: Vec<Range<u64>>,
    /// Correspondences maps, indexed by source
    correspondences: HashMap<String, CorrespondenceMap>,
}

impl Almanac {
    fn new(seeds: Vec<u64>) -> Self {
        // convert the pairs of seeds numbers to ranges
        let seeds_ranges = seeds
            .chunks(2)
            .map(|chunk| match chunk {
                &[start, count] => start..(start + count),
                _ => panic!("invalid seeds spec"),
            })
            .collect();
        Self {
            seeds_ranges,
            correspondences: HashMap::new(),
        }
    }

    fn from_text(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds_line = lines.next().unwrap();
        let seeds = split_numbers(seeds_line.split_once(':').unwrap().1, ' ');
        let mut almanac = Self::new(seeds);
        let mut current_correspondence: Option<CorrespondenceMap> = None;

        for mut line in lines {
            line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.ends_with(':') {
                store_correspondence(&mut current_correspondence, &mut almanac.correspondences);
                let (source, destination) = parse_correspondence_title(line);
                current_correspondence = Some(CorrespondenceMap::new(
                    source.as_str(),
                    destination.as_str(),
                ));
            } else {
                let numbers = split_numbers(line, ' ');
                let destination = numbers[0];
                let source = numbers[1];
                let range = numbers[2];
                if let Some(current_correspondence) = &mut current_correspondence {
                    current_correspondence
                        .mappers
                        .push(Mapper::new(source, destination, range));
                }
            }
        }
        store_correspondence(&mut current_correspondence, &mut almanac.correspondences);

        almanac
    }

    fn map_seeds(&self, source: &str, destination: &str) -> Vec<Range<u64>> {
        let mut current_source = source;
        let mut current_ranges = self.seeds_ranges.clone();
        while current_source != destination {
            let correspondence_map = &self.correspondences[current_source];
            println!(
                "mapping {} to {}",
                current_source, correspondence_map.destination
            );
            current_ranges = current_ranges
                .iter()
                .flat_map(|range| correspondence_map.map_range(range))
                .collect();
            current_source = &correspondence_map.destination;
        }
        current_ranges
    }

    fn find_lowest_location(&self) -> u64 {
        self.map_seeds("seed", "location")
            .iter()
            .map(|range| range.start)
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
        dbg!(&almanac);
        println!("min location: {}", almanac.find_lowest_location());
    }
}

#[cfg(test)]
mod test_mapper {
    use super::*;

    #[test]
    fn test_map() {
        let mapper = Mapper::new(0, 10, 10);
        assert_eq!(mapper.map(0), Some(10));
        assert_eq!(mapper.map(9), Some(19));
        assert_eq!(mapper.map(10), None);
        assert_eq!(mapper.map(11), None);
    }

    #[test]
    fn test_map_range_includes_range() {
        let mapper = Mapper::new(0, 10, 10);
        assert_eq!(mapper.map_range(&(0..10)), Some(vec![10..20]));
        assert_eq!(mapper.map_range(&(1..9)), Some(vec![11..19]));
    }

    #[test]
    fn test_map_range_includes_start() {
        let mapper = Mapper::new(0, 10, 10);
        assert_eq!(mapper.map_range(&(5..15)), Some(vec![15..20, 10..15]));
    }

    #[test]
    fn test_map_range_includes_end() {
        let mapper = Mapper::new(5, 15, 10);
        assert_eq!(mapper.map_range(&(0..10)), Some(vec![0..5, 15..20]));
    }

    #[test]
    fn test_map_range_included_in_range() {
        let mapper = Mapper::new(5, 15, 10);
        assert_eq!(mapper.map_range(&(0..20)), Some(vec![0..5, 15..25, 15..20]));
    }

    #[test]
    fn test_map_range_out_of_bounds() {
        let mapper = Mapper::new(5, 15, 10);
        assert_eq!(mapper.map_range(&(0..5)), None);
        assert_eq!(mapper.map_range(&(15..20)), None);
    }

    #[test]
    fn test_map_range_conserves_length() {
        let mapper = Mapper::new(5, 15, 10);
        let ranges_length = |ranges: &[Range<u64>]| {
            ranges
                .iter()
                .map(|range| range.clone().count())
                .sum::<usize>()
        };
        assert_eq!(ranges_length(&mapper.map_range(&(5..15)).unwrap()), 10);
        assert_eq!(ranges_length(&mapper.map_range(&(0..10)).unwrap()), 10);
        assert_eq!(ranges_length(&mapper.map_range(&(10..20)).unwrap()), 10);
        assert_eq!(ranges_length(&mapper.map_range(&(0..20)).unwrap()), 20);
    }
}
