use aoc_2023_rust_flupke::{split_numbers, Problem};

pub struct Day6;

type Unit = u64;

#[derive(Debug)]
struct Race {
    duration: Unit,
    distance: Unit,
}

impl Race {
    fn distance_for_press(&self, press_duration: Unit) -> Unit {
        let remaining = self.duration - press_duration;
        if remaining > 0 {
            press_duration * remaining
        } else {
            0
        }
    }

    fn count_wins(&self) -> Unit {
        let mut total = 0;
        for press_duration in 1..self.duration {
            if self.distance_for_press(press_duration) > self.distance {
                total += 1;
            }
        }
        total
    }
}

fn parse_races(text: &str) -> Vec<Race> {
    let mut lines = text.lines();
    let durations = split_numbers::<Unit>(
        lines
            .next()
            .expect("durations line")
            .split_once(' ')
            .expect("durations list")
            .1,
    );
    let distances = split_numbers::<Unit>(
        lines
            .next()
            .expect("distances line")
            .split_once(' ')
            .expect("distances list")
            .1,
    );
    durations
        .iter()
        .zip(distances)
        .map(|(duration, distance)| Race {
            duration: *duration,
            distance,
        })
        .collect()
}

fn multiply_total_wins(input_text: &str) -> Unit {
    parse_races(input_text)
        .iter()
        .map(|race| race.count_wins())
        .reduce(|a, b| a * b)
        .unwrap()
}

impl Problem for Day6 {
    fn check(&self) {
        println!(
            "Wins multiplied: {}",
            multiply_total_wins(include_str!("example.txt"))
        );
    }

    fn solve(&self) {
        println!(
            "Wins multiplied: {}",
            multiply_total_wins(include_str!("input.txt"))
        );
    }
}
