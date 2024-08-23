use std::fs::File;
use std::io::{self, Read};

use inquire::Select;

use crate::country::Country;
use crate::file_reader::read_game_state;
use crate::game::Game;

pub fn init(game: &mut Game) -> Result<String, io::Error> {
    let state = read_game_state()?;

    if state.club.is_empty() {
        let country = Select::new("Select club country", Country::all())
            .prompt()
            .expect("No country selected!");

        let selected_club = Select::new(
            "Select club:",
            game.clubs
                .iter()
                .filter(|c| c.country == country)
                .map(|c| c.name.clone())
                .collect(),
        )
        .prompt()
        .expect("No club selected!");

        println!("Welcome to the game! You are managing {}!", selected_club);
        Ok(selected_club)
    } else {
        let selected_club = game
            .clubs
            .iter()
            .find(|c| c.name == state.club)
            .map(|c| c.name.clone())
            .expect("Club not found!");

        println!("Welcome back! You are managing {}!", selected_club);
        Ok(selected_club)
    }
}

pub fn sync_game_state(game_file: &mut File, game: &mut Game) -> Result<(), io::Error> {
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
