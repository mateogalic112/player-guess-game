use crate::club::Club;
use crate::file_reader::{create_clubs, create_players};
use crate::player::Player;

pub struct Game {
    pub clubs: Vec<Club>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            clubs: create_clubs(Club::get_text_file()),
            players: create_players(Player::get_text_file()),
        }
    }
}
