use std::{collections::HashMap, str::FromStr, usize};

use aoc_2023_rust_flupke::Problem;

pub struct Day8;

#[derive(Clone, Debug)]
enum Direction {
    Left = 0,
    Right = 1,
}

#[derive(Default, Debug, Clone)]
struct Instructions {
    instructions: Vec<Direction>,
    current_index: usize,
}

impl Iterator for Instructions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.instructions.get(self.current_index);
        self.current_index = (self.current_index + 1) % self.instructions.len();
        return_value.cloned()
    }
}

impl FromStr for Instructions {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let instructions = input
            .chars()
            .map(|char| match char {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                _ => Err(format!("invalid direction: {:?}", char)),
            })
            .collect::<Result<Vec<Direction>, String>>()?;
        Ok(Instructions {
            instructions,
            ..Instructions::default()
        })
    }
}

#[derive(Debug)]
struct Map {
    instructions: Instructions,
    nodes: HashMap<String, Vec<String>>,
    start_key: String,
    end_key: String,
}

impl Map {
    fn count_steps_to_end(&self) -> usize {
        let mut steps = 1;
        let mut current_key = &self.start_key;
        for direction in self.instructions.clone() {
            let value = &self.nodes[current_key];
            current_key = &value[direction as usize];
            // qqqdbg!(&current_key);
            if *current_key == self.end_key {
                break;
            }
            steps += 1;
        }
        steps
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();

        let instructions = lines.next().unwrap().parse::<Instructions>()?;

        let mut nodes = HashMap::new();
        lines.next();
        for line in lines {
            let (key, value) = line
                .split_once(" = ")
                .ok_or("invalid map line: '=' not found")?;
            let (mut left, mut right) = value
                .split_once(", ")
                .ok_or("invalid map line: ',' not found")?;
            left = &left[1..];
            right = &right[..right.len() - 1];
            nodes.insert(key.to_string(), vec![left.to_string(), right.to_string()]);
        }

        let mut keys = nodes.keys().collect::<Vec<&String>>();
        keys.sort();
        let start_key = keys[0].to_string();
        let end_key = keys[keys.len() - 1].to_string();

        Ok(Map {
            instructions,
            nodes,
            start_key,
            end_key,
        })
    }
}

impl Problem for Day8 {
    fn check(&self) {
        let steps = include_str!("example.txt")
            .parse::<Map>()
            .unwrap()
            .count_steps_to_end();
        println!("steps: {}", steps);
    }

    fn solve(&self) {
        let steps = include_str!("input.txt")
            .parse::<Map>()
            .unwrap()
            .count_steps_to_end();
        println!("steps: {}", steps);
    }
}
