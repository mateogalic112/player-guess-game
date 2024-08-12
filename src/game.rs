use crate::club::Club;
use crate::file_reader::{create_clubs, create_players};
use crate::player::Player;

pub struct Game {
    pub clubs: Vec<Club>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        let mut clubs = create_clubs(Club::get_text_file());
        let players = create_players(Player::get_text_file());

        for (index, player) in players.iter().enumerate() {
            let club_index = index % clubs.len();
            clubs[club_index].squad.push(player.clone());
        }

        Game { clubs, players }
    }
}
