use aoc_2023_rust_flupke::Problem;
use std::collections::HashMap;

pub struct Day1;

struct PlacedDigit {
    digit: char,
    position: usize,
}

fn get_first_and_last_digits(line: &str) -> (char, char) {
    let mut first_digit: Option<PlacedDigit> = None;
    let mut last_digit: Option<PlacedDigit> = None;
    let all_digits = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ]);
    for (search, digit) in &all_digits {
        if let Some(position) = line.find(search) {
            match first_digit {
                None => {
                    first_digit = Some(PlacedDigit {
                        digit: *digit,
                        position,
                    })
                }
                Some(ref mut first_digit) => {
                    if first_digit.position > position {
                        first_digit.digit = *digit;
                        first_digit.position = position;
                    }
                }
            }
        }
        if let Some(position) = line.rfind(search) {
            match last_digit {
                None => {
                    last_digit = Some(PlacedDigit {
                        digit: *digit,
                        position,
                    })
                }
                Some(ref mut last_digit) => {
                    if last_digit.position < position {
                        last_digit.digit = *digit;
                        last_digit.position = position;
                    }
                }
            }
        }
    }
    (first_digit.unwrap().digit, last_digit.unwrap().digit)
}

fn sum_calibration_values(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut combined_digits = String::new();
            let (first_digit, last_digit) = get_first_and_last_digits(line);
            combined_digits.push(first_digit);
            combined_digits.push(last_digit);
            combined_digits.parse::<u32>().unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

impl Problem for Day1 {
    fn check(&self) {
        println!(
            "Calibration values sum: {}",
            sum_calibration_values("src/day_1/example.txt")
        );
    }

    fn solve(&self) {
        println!(
            "Calibration values sum: {}",
            sum_calibration_values("src/day_1/input.txt")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_and_last_digits() {
        assert_eq!(get_first_and_last_digits("1"), ('1', '1'));
        assert_eq!(get_first_and_last_digits("12"), ('1', '2'));
        assert_eq!(get_first_and_last_digits("34"), ('3', '4'));
        assert_eq!(get_first_and_last_digits("56"), ('5', '6'));
        assert_eq!(get_first_and_last_digits("78"), ('7', '8'));
        assert_eq!(get_first_and_last_digits("9one"), ('9', '1'));
        assert_eq!(get_first_and_last_digits("twothree"), ('2', '3'));
        assert_eq!(get_first_and_last_digits("fourfive"), ('4', '5'));
        assert_eq!(get_first_and_last_digits("sixseven"), ('6', '7'));
        assert_eq!(get_first_and_last_digits("eightnine"), ('8', '9'));
        assert_eq!(get_first_and_last_digits("footballone9"), ('1', '9'));
        assert_eq!(get_first_and_last_digits("12345678901"), ('1', '1'));
    }
}
