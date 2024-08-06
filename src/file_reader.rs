use std::fs::{read_to_string, File};
use std::io::ErrorKind;

use crate::player::Player;

pub struct FileReader {
    pub filename: String,
}

impl FileReader {
    fn players_file_reader(&self) -> String {
        let file_content = read_to_string(&self.filename).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(&self.filename).unwrap_or_else(|error| {
                    panic!("Problem creating the file: {error:?}");
                });
                return String::from("");
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        });

        file_content
    }

    pub fn create_players(&self) -> Vec<Player> {
        let players: Vec<Player> = self
            .players_file_reader()
            .lines()
            .filter_map(Player::create_from_line)
            .collect();

        players
    }
}
