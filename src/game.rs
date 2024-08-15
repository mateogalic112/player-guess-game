use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crate::club::Club;
use crate::file_reader::{create_clubs, create_or_open_file, create_players};
use crate::player::Player;
use crate::setup::init;

pub struct Game {
    pub clubs: Vec<Club>,
    pub players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub club: String,
}

impl Game {
    pub fn start(&mut self) -> () {
        let mut game_file = create_or_open_file(Game::get_text_file()).unwrap();

        let club = init(&mut game_file, self);

        loop {
            println!("Input command: ");

            let mut input: String = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: Vec<&str> = input.trim().split(" - ").collect();

            if input.starts_with(&["info::player"]) {
                match self.get_player_info(&input) {
                    Ok(info) => {
                        println!("{}", info);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }

            if input.starts_with(&["info::squad"]) {
                match self.get_squad_info(&input, &club) {
                    Ok(info) => {
                        println!("{}", info);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }

            // ["transfer", "luka modric", "Liverpool", 40]
            if input.starts_with(&["transfer"]) {
                match self.transfer_player(&input.iter().skip(1).cloned().collect()) {
                    Ok(info) => {
                        println!("{}", info);
                        writeln!(game_file, "transfer_player({})", &input[1..].join(", ")).unwrap();
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
    }
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

    pub fn get_player_info(&self, input: &Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
        if input.len() < 2 {
            panic!("Invalid num of args");
        }

        let player: Option<&Player> = Player::find_player_by_name(&self.players, &input[1]);

        match player {
            Some(player) => Ok(player.player_info(player.find_club(&self.clubs))),
            None => {
                panic!("Player not found, try again: ");
            }
        }
    }

    pub fn get_squad_info(
        &self,
        input: &Vec<&str>,
        current_club: &Club,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut club_input = current_club.name.as_str();

        if input.len() > 1 {
            club_input = input[1].trim();
        }

        let club = self
            .clubs
            .iter()
            .find(|c| c.name.to_lowercase() == club_input.to_lowercase().trim());

        match club {
            Some(club) => {
                let mut squad_info = String::new();

                // Print each player in the squad
                for player in &club.squad {
                    squad_info.push_str(&format!("{}\n", player.name));
                }
                Ok(squad_info)
            }
            None => {
                panic!("Club not found, try again: ");
            }
        }
    }

    pub fn transfer_player(
        &mut self,
        input: &Vec<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if input.len() < 3 {
            panic!("Invalid args number");
        }

        let player_name = input[0].trim();
        let new_club_name = input[1].trim();
        let fee: u16 = input[2].trim().parse::<u16>().unwrap();

        match Player::find_player_by_name(&self.players, &player_name) {
            Some(player) => {
                let current_club_name = self
                    .clubs
                    .iter()
                    .find(|c| c.squad.iter().any(|p| p.name == player.name))
                    .unwrap()
                    .name
                    .clone();

                let new_club = self
                    .clubs
                    .iter_mut()
                    .find(|c| c.name.to_lowercase() == new_club_name.to_lowercase())
                    .unwrap();

                let new_club_name = new_club.name.clone();

                if new_club.name == current_club_name {
                    panic!("Player already in this club!")
                }

                if new_club.transfer_budget < fee {
                    panic!("Not enough money!")
                }

                new_club.buy_player(player, fee);

                let current_club = self
                    .clubs
                    .iter_mut()
                    .find(|c| c.name.to_lowercase() == current_club_name.to_lowercase().trim())
                    .unwrap();

                current_club.sell_player(player, fee);

                Ok(format!(
                    "{} bought {} from {} for {} mil.",
                    new_club_name, player.name, current_club_name, fee
                ))
            }
            None => {
                panic!("Player not found, try again: ");
            }
        }
    }

    pub fn get_text_file() -> &'static str {
        const GAME_FILE: &str = "game.txt";
        GAME_FILE
    }

    pub fn get_json_file() -> &'static str {
        const GAME_JSON_FILE: &str = "game.json";
        GAME_JSON_FILE
    }
}
