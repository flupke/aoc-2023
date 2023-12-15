use aoc_2023_rust_flupke::Problem;
use clap::{Parser, ValueEnum};
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

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
        _ => panic!("Day {} not implemented", args.day),
    };
    match args.command {
        Command::Solve => module.solve(),
        Command::Check => module.check(),
    }
}
