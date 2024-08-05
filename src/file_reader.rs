use std::fs::{read_to_string, File};
use std::io::ErrorKind;

use crate::player::Player;

pub struct FileReader {
    pub filename: String,
}

impl FileReader {
    fn players_file_reader(&self) -> String {
        let file_content = match read_to_string(&self.filename) {
            Ok(content) => content,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(&self.filename) {
                    Ok(_fc) => String::from(""),
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}");
                }
            },
        };

        file_content
    }

    pub fn create_players(&self) -> Vec<Player> {
        let file_content = self.players_file_reader();
        let players: Vec<Player> = file_content
            .lines()
            .filter_map(Player::create_from_line)
            .collect();

        players
    }
}
