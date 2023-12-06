use aoc_2023_rust_flupke::Problem;

pub struct Day2;

fn is_possible_game(line: &str, max_red: u8, max_green: u8, max_blue: u8) -> (u16, bool) {
    let (prefix, suffix) = line.split_once(":").unwrap();
    let game_id = prefix.split_once(" ").unwrap().1.parse::<u16>().unwrap();
    for draft in suffix.split(";") {
        for draft_entry in draft.split(",") {
            let (number, color) = draft_entry.trim().split_once(" ").unwrap();
            let number = number.parse::<u8>().unwrap();
            match color {
                "red" => {
                    if number > max_red {
                        return (game_id, false);
                    }
                }
                "green" => {
                    if number > max_green {
                        return (game_id, false);
                    }
                }
                "blue" => {
                    if number > max_blue {
                        return (game_id, false);
                    }
                }
                _ => panic!("Unknown color {}", color),
            }
        }
    }
    return (game_id, true);
}

impl Problem for Day2 {
    fn check(&self) {
        panic!("Not implemented");
    }

    fn solve(&self) {
        let mut possible_games_sum = 0;
        std::fs::read_to_string("src/day_2/input.txt")
            .unwrap()
            .lines()
            .for_each(|line| {
                let (game_id, is_possible) = is_possible_game(line, 12, 13, 14);
                if is_possible {
                    possible_games_sum += game_id;
                }
            });
        println!("Possible games sum: {}", possible_games_sum);
    }
}
