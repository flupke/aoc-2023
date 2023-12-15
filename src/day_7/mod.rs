use std::{str::FromStr, usize};

use aoc_2023_rust_flupke::Problem;
use itertools::Itertools;

pub struct Day7;

const CARDS: &str = "J23456789TQKA";

fn card_value(card: char) -> Result<usize, String> {
    CARDS.find(card).ok_or(format!("invalid card: {:?}", card))
}

type HandValue = (HandType, usize, usize, usize, usize, usize);

struct Hand {
    cards: Vec<usize>,
}

#[derive(Debug, PartialOrd, Eq, Ord, PartialEq, Clone, Copy)]
enum HandType {
    HighCard = 0,
    Pair = 1,
    TwoPairs = 2,
    Triple = 3,
    FullHouse = 4,
    Quad = 5,
    Quint = 6,
}

impl Hand {
    fn value(&self) -> HandValue {
        (
            self.best_hand_type(),
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4],
        )
    }

    fn best_hand_type(&self) -> HandType {
        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort_by(|a, b| b.cmp(a));
        let jokers_chain = find_jokers_chain(&sorted_cards);
        if let Some((start, length)) = jokers_chain {
            (0..length)
                .map(|_| 1..CARDS.len())
                .multi_cartesian_product()
                .map(|joker_cards| {
                    let mut cards = sorted_cards.clone();
                    for (index, card) in joker_cards.iter().enumerate() {
                        cards[start + index] = *card;
                    }
                    cards.sort_by(|a, b| b.cmp(a));
                    hand_type(&cards)
                })
                .max()
                .unwrap()
        } else {
            hand_type(&sorted_cards)
        }
    }
}

fn hand_type(sorted_cards: &Vec<usize>) -> HandType {
    let mut num_pairs = 0;
    let mut num_triples = 0;
    let mut num_quads = 0;
    let mut num_quints = 0;
    let mut prev_card = usize::MAX;
    let mut chain_length = 1;

    let mut update_counts = |chain_length: &usize| match chain_length {
        1 => {}
        2 => num_pairs += 1,
        3 => num_triples += 1,
        4 => num_quads += 1,
        5 => num_quints += 1,
        _ => panic!("too many cards!"),
    };

    for card in sorted_cards {
        if *card == prev_card {
            chain_length += 1;
        } else {
            update_counts(&chain_length);
            chain_length = 1;
        }
        prev_card = *card
    }
    update_counts(&chain_length);

    match (num_quints, num_quads, num_triples, num_pairs) {
        (1, _, _, _) => HandType::Quint,
        (_, 1, _, _) => HandType::Quad,
        (_, _, 1, 1) => HandType::FullHouse,
        (_, _, 1, _) => HandType::Triple,
        (_, _, _, 2) => HandType::TwoPairs,
        (_, _, _, 1) => HandType::Pair,
        _ => HandType::HighCard,
    }
}

fn find_jokers_chain(sorted_cards: &[usize]) -> Option<(usize, usize)> {
    let mut chain_length = 0;
    let mut chain_start = None;
    for (index, card) in sorted_cards.iter().enumerate() {
        if *card == 0 {
            if chain_start.is_none() {
                chain_start = Some(index);
            }
            chain_length += 1;
        }
    }
    chain_start.map(|start| (start, chain_length))
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cards = input
            .chars()
            .map(card_value)
            .collect::<Result<Vec<usize>, String>>()?;
        if cards.len() != 5 {
            Err(format!("hand is not 5 cards: {:?}", input))
        } else {
            Ok(Hand { cards })
        }
    }
}

struct Bid {
    hand: Hand,
    bid: usize,
}

impl FromStr for Bid {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = line
            .split_once(' ')
            .ok_or(format!("invalid bid line: {:?}", line))?;
        let hand = hand_str.parse::<Hand>()?;
        let bid = bid_str
            .parse::<usize>()
            .map_err(|err| format!("cannot parse bid: {:?}", err))?;
        Ok(Bid { hand, bid })
    }
}

struct Game {
    bids: Vec<Bid>,
}

impl Game {
    fn total_winnings(&self) -> usize {
        let mut values = self
            .bids
            .iter()
            .map(|bid| bid.hand.value())
            .enumerate()
            .collect::<Vec<(usize, HandValue)>>();
        values.sort_by_key(|(_, value)| *value);
        values
            .iter()
            .enumerate()
            .map(|(rank, (index, _))| (rank + 1) * self.bids[*index].bid)
            .sum()
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let bids = input
            .lines()
            .map(|line| line.parse::<Bid>())
            .collect::<Result<Vec<Bid>, String>>()?;
        Ok(Game { bids })
    }
}

impl Problem for Day7 {
    fn check(&self) {
        let total_winnings = include_str!("example.txt")
            .parse::<Game>()
            .unwrap()
            .total_winnings();
        println!("total winnings: {}", total_winnings);
    }

    fn solve(&self) {
        let total_winnings = include_str!("input.txt")
            .parse::<Game>()
            .unwrap()
            .total_winnings();
        println!("total winnings: {}", total_winnings);
    }
}

#[cfg(test)]
mod test_mapper {
    use super::*;

    #[test]
    fn test_hand_type_ordering() {
        assert!(HandType::HighCard < HandType::Pair);
        assert!(HandType::FullHouse == HandType::FullHouse);
    }

    #[test]
    fn test_hand_value_ordering() {
        let hand1 = "AAAQ4".parse::<Hand>().unwrap();
        let hand2 = "AAAQ3".parse::<Hand>().unwrap();
        assert!(hand1.value() > hand2.value());

        let hand1 = "2AA2A".parse::<Hand>().unwrap();
        let hand2 = "AA3A3".parse::<Hand>().unwrap();
        assert!(hand1.value() < hand2.value());
    }
}
