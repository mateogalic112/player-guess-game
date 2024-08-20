use std::fs::File;
use std::io::Read;

use inquire::{InquireError, Select};
use serde_json::json;

use crate::club::Club;
use crate::country::Country;
use crate::file_reader::{read_game_state, update_game_state};
use crate::game::{Game, GameState};

fn select_country() -> Option<Country> {
    let country_options: Vec<Country> = vec![Country::England, Country::Spain];

    let country_ans: Result<Country, InquireError> =
        Select::new("Select club country?", country_options).prompt();

    match country_ans {
        Ok(choice) => {
            println!("{}! That's mine too!", choice);
            Some(choice)
        }
        Err(_) => {
            println!("There was an error, please try again");
            None
        }
    }
}

fn select_club(country: &Country, clubs: &Vec<Club>) -> Option<Club> {
    let clubs = match country {
        Country::England => clubs
            .into_iter()
            .filter(|c| c.country == Country::England)
            .collect(),
        Country::Spain => clubs
            .into_iter()
            .filter(|c| c.country == Country::Spain)
            .collect(),
    };

    let clubs_ans: Result<&Club, InquireError> = Select::new("Select club:", clubs).prompt();

    match clubs_ans {
        Ok(choice) => {
            println!("{}! That's mine too!", choice.name);
            Some(choice.clone())
        }
        Err(_) => {
            println!("There was an error, please try again");
            None
        }
    }
}

pub fn init(game_file: &mut File, game: &mut Game) -> Club {
    let mut buf = String::new();
    game_file.read_to_string(&mut buf).unwrap();

    buf.lines().for_each(|line| execute_command(line, game));

    let state = read_game_state().unwrap();
    let club: Club = match state.club.is_empty() {
        true => {
            let country: Country = select_country().unwrap();
            let selected_club = select_club(&country, &game.clubs).unwrap();

            update_game_state(&json!({"club": selected_club.name}))
                .unwrap_or_else(|e| println!("e: {}", e));

            println!(
                "Welcome to the game! You are now managing {}!",
                selected_club.name
            );

            selected_club
        }
        false => {
            let state: GameState = read_game_state().unwrap();
            let selected_club = game
                .clubs
                .iter()
                .find(|c| c.name == state.club)
                .unwrap()
                .clone();
            println!("Welcome back! You are managing {}!", selected_club.name);
            selected_club
        }
    };

    club
}

fn execute_command(command: &str, game: &mut Game) {
    // Example: transfer_player(Falcao, Arsenal, 10)
    if command.starts_with("transfer_player") {
        let args_str = command
            .trim_start_matches("transfer_player(")
            .trim_end_matches(')');
        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

        game.transfer_player(&args).unwrap();
    } else {
        eprintln!("Invalid command: {}", command);
    }
}
