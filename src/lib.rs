use std::str::FromStr;

pub trait Problem {
    fn check(&self);
    fn solve(&self);
}

pub fn split_numbers<T: FromStr>(line: &str) -> Vec<T> {
    line.split(' ')
        .filter_map(|n| n.trim().parse::<T>().ok())
        .collect::<Vec<T>>()
}
