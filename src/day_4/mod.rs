use aoc_2023_rust_flupke::Problem;

pub struct Day4;

struct Card {
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

fn split_numbers(line: &str) -> Vec<u8> {
    line.split(" ")
        .filter_map(|n| n.trim().parse::<u8>().ok())
        .collect::<Vec<u8>>()
}

impl Card {
    fn new(line: &str) -> Self {
        let card_content = line.split_once(":").unwrap().1;
        let (winning_numbers_text, numbers_text) = card_content.split_once("|").unwrap();
        let winning_numbers = split_numbers(winning_numbers_text);
        let numbers = split_numbers(numbers_text);
        Self {
            winning_numbers,
            numbers,
        }
    }

    fn value(&self) -> u32 {
        let mut value = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                if value == 0 {
                    value = 1;
                } else {
                    value *= 2;
                }
            }
        }
        value
    }
}

fn sum_winning_numbers(input: &str) -> u32 {
    input.lines().map(Card::new).map(|card| card.value()).sum()
}

impl Problem for Day4 {
    fn check(&self) {
        let input = std::fs::read_to_string("src/day_4/example.txt").unwrap();
        println!("Total score: {}", sum_winning_numbers(&input));
    }

    fn solve(&self) {
        let input = std::fs::read_to_string("src/day_4/input.txt").unwrap();
        println!("Total score: {}", sum_winning_numbers(&input));
    }
}
