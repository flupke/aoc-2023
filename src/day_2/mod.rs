use aoc_2023_rust_flupke::Problem;

pub struct Day2;

struct Draft {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draft {
    fn new() -> Draft {
        Draft {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse_draft(draft: &str) -> Draft {
    let mut result = Draft::new();
    for draft_entry in draft.split(',') {
        let (number, color) = draft_entry.trim().split_once(' ').unwrap();
        let number = number.parse::<u32>().unwrap();
        match color {
            "red" => result.red += number,
            "green" => result.green += number,
            "blue" => result.blue += number,
            _ => panic!("Unknown color {}", color),
        }
    }
    result
}

fn get_game_power(line: &str) -> u32 {
    let suffix = line.split_once(':').unwrap().1;
    let mut min_draft = Draft::new();
    for draft in suffix.split(';') {
        let draft = parse_draft(draft);
        if draft.red > min_draft.red {
            min_draft.red = draft.red;
        }
        if draft.green > min_draft.green {
            min_draft.green = draft.green;
        }
        if draft.blue > min_draft.blue {
            min_draft.blue = draft.blue;
        }
    }
    min_draft.power()
}

impl Problem for Day2 {
    fn check(&self) {
        panic!("Not implemented");
    }

    fn solve(&self) {
        let powers_sum = std::fs::read_to_string("src/day_2/input.txt")
            .unwrap()
            .lines()
            .map(get_game_power)
            .reduce(|a, b| a + b)
            .unwrap();
        println!("Possible games sum: {}", powers_sum);
    }
}
