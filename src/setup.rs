use std::fs::File;
use std::io::{Error, Read};

use inquire::{InquireError, Select};

use crate::club::Club;
use crate::country::Country;
use crate::file_reader::read_game_state;
use crate::game::Game;

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

pub fn init<'a>(game: &'a mut Game) -> Result<Club, Error> {
    let state = read_game_state()?;

    match state.club.is_empty() {
        true => {
            let country = select_country().expect("No country selected!");
            let selected_club = select_club(country, &game.clubs).expect("No club selected!");

            println!(
                "Welcome to the game! You are now managing {}!",
                selected_club.name
            );
            Ok(selected_club.clone())
        }
        false => {
            let selected_club = game
                .clubs
                .iter()
                .find(|c| c.name == state.club)
                .expect("Club not found!");

            println!("Welcome back! You are managing {}!", selected_club.name);
            Ok(selected_club.clone())
        }
    }
}

pub fn sync_game_state(game_file: &mut File, game: &mut Game) -> Result<(), Error> {
    let mut buf = String::new();
    game_file.read_to_string(&mut buf)?;
    buf.lines().for_each(|line| execute_command(line, game));
    Ok(())
}

fn execute_command(command: &str, game: &mut Game) {
    // Example: transfer_player(Falcao, Arsenal, 10)
    if command.starts_with("transfer_player") {
        let args_str = command
            .trim_start_matches("transfer_player(")
            .trim_end_matches(')');
        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

        game.transfer_player(&args).unwrap_or_else(|err| err);
    } else {
        eprintln!("Invalid command: {}", command);
    }
}

/// Capitalizes the first character in s.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}
