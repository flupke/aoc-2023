use aoc_2023_rust_flupke::Problem;

#[derive(Clone, Debug)]
pub struct Day1;

fn sum_calibration_values(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            let first = digits[0];
            let last = digits[digits.len() - 1];
            let mut s = String::new();
            s.push(first);
            s.push(last);
            s.parse::<u32>().unwrap()
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
