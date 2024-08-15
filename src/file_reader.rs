use std::fs::{read_to_string, File, OpenOptions};
use std::io::{BufReader, ErrorKind};

use serde_json::Value;

use crate::club::Club;
use crate::game::{Game, GameState};
use crate::player::Player;

fn get_file_content(filename: &str) -> String {
    let file_content = read_to_string(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).expect("Failed to create file");
            return String::from("");
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    file_content
}

pub fn create_players(filename: &str) -> Vec<Player> {
    let players: Vec<Player> = get_file_content(filename)
        .lines()
        .filter_map(Player::new)
        .collect();

    players
}

pub fn create_clubs(filename: &str) -> Vec<Club> {
    let clubs: Vec<Club> = get_file_content(filename)
        .lines()
        .filter_map(Club::new)
        .collect();

    clubs
}

pub fn create_or_open_file(filename: &str) -> Result<File, std::io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true) // Open the file in write mode
        .create(true) // Create the file if it does not exist
        .append(true) // Append to the file if it exists
        .open(filename)?; // Open or create the file

    Ok(file)
}

pub fn read_game_state() -> Result<GameState, std::io::Error> {
    // Open the file
    let file = File::open(Game::get_json_file())?;

    // Create a buffered reader for efficient reading
    let reader = BufReader::new(file);

    // Parse the JSON file into the `Club` struct
    let state: GameState = serde_json::from_reader(reader)?;

    Ok(state)
}

pub fn update_game_state(json_data: &Value) -> Result<(), std::io::Error> {
    // Open the file
    let file = OpenOptions::new().write(true).open(Game::get_json_file())?;

    serde_json::to_writer_pretty(file, json_data)?;

    Ok(())
}
