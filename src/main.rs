use aoc_2023_rust_flupke::Problem;
use clap::{Parser, ValueEnum};
mod common;
mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_enum)]
    command: Command,

    #[arg()]
    day: u8,
}

#[derive(ValueEnum, Clone, Debug)]
enum Command {
    Check,
    Solve,
}

fn main() {
    let args = Cli::parse();
    let module: Box<dyn Problem> = match args.day {
        1 => Box::new(day_1::Day1),
        2 => Box::new(day_2::Day2),
        3 => Box::new(day_3::Day3),
        4 => Box::new(day_4::Day4),
        5 => Box::new(day_5::Day5),
        6 => Box::new(day_6::Day6),
        7 => Box::new(day_7::Day7),
        8 => Box::new(day_8::Day8),
        9 => Box::new(day_9::Day9),
        10 => Box::new(day_10::Day10),
        11 => Box::new(day_11::Day11),
        12 => Box::new(day_12::Day12),
        13 => Box::new(day_13::Day13),
        14 => Box::new(day_14::Day14),
        15 => Box::new(day_15::Day15),
        16 => Box::new(day_16::Day16),
        _ => panic!("Day {} not implemented", args.day),
    };
    match args.command {
        Command::Solve => module.solve(),
        Command::Check => module.check(),
    }
}
