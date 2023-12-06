use aoc_2023_rust_flupke::Problem;
use clap::{Parser, ValueEnum};
mod day_1;

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
    let module = match args.day {
        1 => Box::new(day_1::Day1),
        _ => panic!("Day {} not implemented", args.day),
    };
    match args.command {
        Command::Solve => module.solve(),
        Command::Check => module.check(),
    }
}
