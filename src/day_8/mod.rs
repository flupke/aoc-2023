use std::{collections::HashMap, str::FromStr, usize};

use aoc_2023_rust_flupke::Problem;

pub struct Day8;

#[derive(Clone, Debug, Copy)]
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
}

impl Map {
    fn count_common_steps_to_end(&self) -> usize {
        self.nodes
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|key| self.count_steps_to_end(key))
            .reduce(least_common_divisor)
            .unwrap()
    }

    fn count_steps_to_end(&self, start_key: &str) -> usize {
        let mut steps = 1;
        let mut current_key = start_key;
        for direction in self.instructions.clone() {
            let value = &self.nodes[current_key];
            current_key = &value[direction as usize];
            if current_key.ends_with('Z') {
                break;
            }
            steps += 1;
        }
        steps
    }
}

fn least_common_divisor(a: usize, b: usize) -> usize {
    a * b / greatest_common_factor(a, b)
}

fn greatest_common_factor(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
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

        Ok(Map {
            instructions,
            nodes,
        })
    }
}

impl Problem for Day8 {
    fn check(&self) {
        let steps = include_str!("example.txt")
            .parse::<Map>()
            .unwrap()
            .count_common_steps_to_end();
        println!("steps: {}", steps);
    }

    fn solve(&self) {
        let steps = include_str!("input.txt")
            .parse::<Map>()
            .unwrap()
            .count_common_steps_to_end();
        println!("steps: {}", steps);
    }
}
