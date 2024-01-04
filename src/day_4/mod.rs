use aoc_2023_rust_flupke::{split_numbers, Problem};

pub struct Day4;

#[derive(Debug)]
struct Card {
    _id: u8,
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn new(line: &str) -> Self {
        let (id_text, card_content) = line.split_once(':').unwrap();
        let (winning_numbers_text, numbers_text) = card_content.split_once('|').unwrap();
        let winning_numbers = split_numbers(winning_numbers_text, ' ');
        let numbers = split_numbers(numbers_text, ' ');
        let id = id_text
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<u8>()
            .unwrap();
        Self {
            _id: id,
            winning_numbers,
            numbers,
        }
    }

    fn matches_count(&self) -> usize {
        let mut count = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                count += 1;
            }
        }
        count
    }
}

fn count_scratch_cards(input: &str) -> u32 {
    let cards: Vec<Card> = input.lines().map(Card::new).collect();
    do_count_scratch_cards(cards.as_slice(), cards.len(), 0)
}

fn do_count_scratch_cards(cards: &[Card], up_to_index: usize, count: u32) -> u32 {
    let mut new_count = count;
    for (index, card) in cards[..up_to_index].iter().enumerate() {
        new_count += 1;
        let matches = card.matches_count();
        if matches > 0 {
            let extra_cards = &cards[index + 1..];
            new_count += do_count_scratch_cards(extra_cards, matches, count);
        }
    }
    new_count
}

impl Problem for Day4 {
    fn check(&self) {
        let input = std::fs::read_to_string("src/day_4/example.txt").unwrap();
        println!("Total cards: {}", count_scratch_cards(&input));
    }

    fn solve(&self) {
        let input = std::fs::read_to_string("src/day_4/input.txt").unwrap();
        println!("Total cards: {}", count_scratch_cards(&input));
    }
}
