use std::fs::File;
use std::io::Read;

use inquire::{InquireError, Select};
use serde_json::json;

use crate::club::Club;
use crate::country::Country;
use crate::file_reader::{read_game_state, update_game_state};
use crate::game::{Game, GameState};

fn select_country() -> Result<Country, InquireError> {
    Select::new("Select club country?", Country::all()).prompt()
}

fn select_club<'a>(country: Country, clubs: &'a Vec<Club>) -> Result<&'a Club, InquireError> {
    Select::new(
        "Select club:",
        Country::get_clubs_from_country(country, clubs),
    )
    .prompt()
}

pub fn init<'a>(game_file: &'a mut File, game: &'a mut Game) -> Club {
    let mut buf = String::new();
    game_file.read_to_string(&mut buf).unwrap();

    buf.lines().for_each(|line| execute_command(line, game));

    let state = read_game_state().unwrap();

    match state.club.is_empty() {
        true => {
            let country = select_country().expect("No country selected!");
            let selected_club = select_club(country, &game.clubs).expect("No club selected!");

            update_game_state(&json!({"club": selected_club.name}))
                .unwrap_or_else(|e| println!("e: {}", e));

            println!(
                "Welcome to the game! You are now managing {}!",
                selected_club.name
            );

            selected_club.clone()
        }
        false => {
            let state: GameState = read_game_state().unwrap();
            let selected_club = game.clubs.iter().find(|c| c.name == state.club).unwrap();
            println!("Welcome back! You are managing {}!", selected_club.name);
            selected_club.clone()
        }
    }
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
