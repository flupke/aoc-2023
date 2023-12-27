use itertools::Itertools;
use std::{fmt::Display, usize};

use aoc_2023_rust_flupke::{split_numbers, Problem};

pub struct Day12;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Spring {
    Unknown,
    Operational,
    Damaged,
}

#[allow(dead_code)]
impl Spring {
    fn from_char(input: char) -> Self {
        match input {
            '?' => Self::Unknown,
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => panic!("invalid tile: {:?}", input),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Self::Unknown => '?',
            Self::Operational => '.',
            Self::Damaged => '#',
        }
    }

    fn potential_values(&self) -> Option<Vec<Self>> {
        match self {
            Self::Unknown => Some(vec![Self::Operational, Self::Damaged]),
            _ => None,
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
        }
    }
}

struct Record {
    springs: Vec<Spring>,
    damaged_springs: Vec<usize>,
}

#[allow(dead_code)]
impl Record {
    fn from_str(input: &str) -> Self {
        let (springs_str, damaged_springs_str) = input.split_once(' ').unwrap();
        let springs = springs_str.chars().map(Spring::from_char).collect();
        let damaged_springs = split_numbers(damaged_springs_str, ',');
        Self {
            springs,
            damaged_springs,
        }
    }

    fn replace_springs(&self, replacements: &Vec<&(usize, Spring)>) -> Self {
        let mut springs = self.springs.clone();
        for (index, spring) in replacements {
            springs[*index] = *spring;
        }
        Record {
            springs,
            damaged_springs: self.damaged_springs.clone(),
        }
    }

    fn format(&self) -> String {
        format!(
            "{} {}",
            self.springs
                .iter()
                .map(|spring| spring.as_char())
                .collect::<String>(),
            self.damaged_springs
                .iter()
                .map(|&value| value.to_string())
                .join(",")
        )
    }

    fn damaged_springs_description(&self) -> Vec<usize> {
        let mut num_damaged = 0;
        let mut result = Vec::new();

        for spring in &self.springs {
            match spring {
                Spring::Damaged => num_damaged += 1,
                Spring::Operational => {
                    if num_damaged > 0 {
                        result.push(num_damaged);
                        num_damaged = 0;
                    }
                }
                _ => panic!("invalid spring: {:?}", spring),
            }
        }
        if num_damaged > 0 {
            result.push(num_damaged);
        }

        result
    }

    fn self_check(&self) -> bool {
        let damaged_springs = self.damaged_springs_description();
        damaged_springs == self.damaged_springs
    }

    fn arrangements(&self) -> usize {
        let arrangements_input: Vec<Vec<(usize, Spring)>> = self
            .springs
            .iter()
            .enumerate()
            .flat_map(|(index, &value)| {
                value.potential_values().map(|potential_values| {
                    potential_values
                        .iter()
                        .map(|&value| (index, value))
                        .collect()
                })
            })
            .collect();
        let mut arragements = 0;
        for replacements in arrangements_input.iter().multi_cartesian_product() {
            let record = self.replace_springs(&replacements);
            if record.self_check() {
                arragements += 1;
            }
        }
        arragements
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
            .map(|record| record.arrangements())
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
