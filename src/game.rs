use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crate::club::Club;
use crate::file_reader::{create_entities, create_or_open_file};
use crate::player::Player;
use crate::setup::{init, sync_game_state};

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
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut game_file = create_or_open_file(Game::get_text_file())?;

        sync_game_state(&mut game_file, self)?;

        let club = init(self);

        loop {
            println!("Input command: ");

            let mut input: String = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: Vec<&str> = input.trim().split(" - ").collect();

            if input.starts_with(&["info::player"]) {
                println!("{}", self.get_player_info(&input));
            }

            if input.starts_with(&["info::squad"]) {
                println!("{}", self.get_squad_info(&input, &club));
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
        let clubs = create_entities::<Club>(Club::get_text_file());
        let mut players = create_entities::<Player>(Player::get_text_file());

        for (index, player) in players.iter_mut().enumerate() {
            let club_index = index % clubs.len();
            player.club = Some(clubs[club_index].name.clone());
        }

        Game { clubs, players }
    }

    pub fn get_player_info(&self, input: &Vec<&str>) -> String {
        if input.len() < 2 {
            return String::from("Invalid num of args");
        }

        let player: Option<&Player> = Player::find_player_by_name(&self.players, &input[1]);

        match player {
            Some(player) => player.to_string(),
            None => String::from("Player not found, try again: "),
        }
    }

    pub fn get_squad_info(&self, input: &Vec<&str>, current_club: &Club) -> String {
        let mut club_input = current_club.name.as_str();

        if input.len() > 1 {
            club_input = input[1].trim();
        }

        let mut squad_info = String::new();

        // Print each player in the squad
        for player in self
            .players
            .iter()
            .filter(|p| p.club == Some(club_input.to_string()))
        {
            squad_info.push_str(&format!("{}\n", &player.name));
        }

        squad_info
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

        let fee = input[2].trim().parse::<u16>();
        let fee = match fee {
            Ok(value) => value,
            Err(_) => Err("Invalid fee".to_string())?,
        };

        let player = self
            .players
            .iter_mut()
            .find(|player| player.name.eq_ignore_ascii_case(player_name))
            .ok_or_else(|| "Player not found")?;

        match &player.club {
            Some(club) if club.eq_ignore_ascii_case(new_club_name) => {
                return Err("Player already in this club!".to_string())?;
            }
            _ => (),
        }

        let new_club = self
            .clubs
            .iter_mut()
            .find(|club| club.name.eq_ignore_ascii_case(new_club_name))
            .ok_or_else(|| "New club not found")?;

        if new_club.transfer_budget < fee {
            return Err("Not enough money!".to_string())?;
        }

        new_club.transfer_budget -= fee;

        let current_club = self
            .clubs
            .iter_mut()
            .find(|club| {
                club.name
                    .eq_ignore_ascii_case(player.club.as_ref().unwrap().as_str())
            })
            .ok_or_else(|| "Current club not found")?;

        current_club.transfer_budget += fee;

        Ok(format!(
            "{} bought {} from {} for {} mil.",
            new_club_name, player.name, current_club.name, fee
        ))
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
