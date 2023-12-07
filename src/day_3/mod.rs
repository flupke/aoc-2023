use std::collections::HashSet;

use aoc_2023_rust_flupke::Problem;

pub struct Day3;

struct Schematic {
    lines: Vec<String>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct PlacedPart {
    x_start: usize,
    x_end: usize,
    y: usize,
    value: String,
}

#[derive(Debug, Clone)]
struct PlacedSymbol {
    x: usize,
    y: usize,
    symbol: char,
}

impl Schematic {
    fn new(text: &str) -> Self {
        let lines = text
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let width = lines[0].len();
        let height = lines.len();
        Self {
            lines,
            width,
            height,
        }
    }

    fn parse(&self) -> (Vec<PlacedPart>, Vec<PlacedSymbol>) {
        let mut parts = Vec::new();
        let mut symbols = Vec::new();
        let mut current_part: Option<PlacedPart> = None;
        for y in 0..self.height {
            for x in 0..self.width {
                let char = self.lines[y].chars().nth(x).unwrap();
                if char.is_digit(10) {
                    match current_part {
                        None => {
                            current_part = Some(PlacedPart {
                                x_start: x,
                                x_end: x,
                                y,
                                value: char.to_string(),
                            })
                        }
                        Some(ref mut current_part) => {
                            current_part.x_end = x;
                            current_part.value.push(char);
                        }
                    }
                } else if char == '.' {
                    if current_part.is_some() {
                        parts.push(current_part.clone().unwrap());
                        current_part = None;
                    }
                } else {
                    if current_part.is_some() {
                        parts.push(current_part.clone().unwrap());
                        current_part = None;
                    }
                    symbols.push(PlacedSymbol { x, y, symbol: char });
                }
            }
            if current_part.is_some() {
                parts.push(current_part.clone().unwrap());
                current_part = None;
            }
        }
        (parts, symbols)
    }
}

fn get_parts_sum(text: &str) -> u32 {
    let schematic = Schematic::new(text);
    let (parts, symbols) = schematic.parse();
    let mut touched_parts = HashSet::new();
    for symbol in symbols {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let x = symbol.x as i32 + dx;
                let y = symbol.y as i32 + dy;
                for part in &parts {
                    if part.x_start as i32 <= x && x <= part.x_end as i32 && part.y as i32 == y {
                        touched_parts.insert(part);
                    }
                }
            }
        }
    }
    touched_parts
        .iter()
        .map(|part| part.value.parse::<u32>().unwrap())
        .sum()
}

impl Problem for Day3 {
    fn check(&self) {
        let text = std::fs::read_to_string("src/day_3/example.txt").unwrap();
        let parts_sum = get_parts_sum(&text);
        println!("Parts sum: {}", parts_sum);
    }

    fn solve(&self) {
        let text = std::fs::read_to_string("src/day_3/input.txt").unwrap();
        let parts_sum = get_parts_sum(&text);
        println!("Parts sum: {}", parts_sum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let schematic = Schematic::new("1..2");
        let (parts, symbols) = schematic.parse();
        assert_eq!(symbols.len(), 0);
        assert_eq!(parts.len(), 2);
    }
}
