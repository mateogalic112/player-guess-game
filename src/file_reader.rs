use std::fs::{read_to_string, File, OpenOptions};
use std::io::ErrorKind;

use crate::club::Club;
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
