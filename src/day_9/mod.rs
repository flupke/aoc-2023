use aoc_2023_rust_flupke::{split_numbers, Problem};

pub struct Day9;

struct Data {
    series: Vec<Vec<i64>>,
}

impl Data {
    fn parse(input: &str) -> Self {
        let series = input.lines().map(|line| split_numbers(line, ' ')).collect();
        Data { series }
    }

    fn predictions_sum(&self) -> i64 {
        self.series.iter().map(|values| predict(values)).sum()
    }
}

fn predict(values: &[i64]) -> i64 {
    let mut all_diffs = Vec::new();
    let mut input = values.to_vec();
    loop {
        let diffs = input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>();
        if diffs.iter().all(|v| *v == 0) {
            break;
        }
        all_diffs.push(diffs.clone());
        input = diffs;
    }
    let mut prediction = 0;
    for diff in all_diffs.iter().rev() {
        prediction = diff.first().unwrap() - prediction;
    }
    values.first().unwrap() - prediction
}

impl Problem for Day9 {
    fn check(&self) {
        let sum = Data::parse(include_str!("example.txt")).predictions_sum();
        println!("sum: {}", sum);
    }

    fn solve(&self) {
        let sum = Data::parse(include_str!("input.txt")).predictions_sum();
        println!("sum: {}", sum);
    }
}
