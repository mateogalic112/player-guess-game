use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, Write};

use crate::club::Club;
use crate::file_reader::{create_entities, create_or_open_file, update_game_state};
use crate::player::Player;
use crate::setup::{capitalize, init, sync_game_state};

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
    pub fn start(&mut self) -> Result<(), io::Error> {
        let mut game_file = create_or_open_file(Game::get_text_file())?;
        sync_game_state(&mut game_file, self)?;

        let selected_club = init(self)?;
        update_game_state(&json!({"club": selected_club}))?;

        loop {
            println!("Input command: ");

            let mut input: String = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim().split(" - ").collect::<Vec<&str>>();

            if input.starts_with(&["info::player"]) {
                println!("{}", self.get_player_info(&input));
            }

            if input.starts_with(&["info::squad"]) {
                println!("{}", self.get_squad_info(&input, &selected_club));
            }

            // ["transfer", "luka modric", "Liverpool", 40]
            if input.starts_with(&["transfer"]) {
                match self.transfer_player(&input[1..].to_vec()) {
                    Ok(transfer_info) => {
                        println!("{}", transfer_info);
                        writeln!(game_file, "transfer_player({})", &input[1..].join(", "))?;
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

        match Player::find_player_by_name(&self.players, &input[1]) {
            Some(player) => player.to_string(),
            None => String::from("Player not found, try again: "),
        }
    }

    pub fn get_squad_info(&self, input: &Vec<&str>, current_club: &str) -> String {
        let mut club_input = current_club;

        if input.len() > 1 {
            club_input = input[1].trim();
        }

        // Print each player in the squad
        self.players
            .iter()
            .filter(|p| {
                p.club
                    .as_deref()
                    .map_or(false, |club| club.eq_ignore_ascii_case(club_input))
            })
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn transfer_player(&mut self, input: &Vec<&str>) -> Result<String, String> {
        if input.len() < 3 {
            return Err(String::from("Invalid args number"));
        }

        let player_name = input[0].trim();
        let new_club_name = capitalize(input[1].trim());

        let fee = input[2]
            .trim()
            .parse::<u16>()
            .map_err(|_| "Invalid fee. Fee must be a positive integer.")?;

        let player = self
            .players
            .iter_mut()
            .find(|player| player.name.eq_ignore_ascii_case(player_name))
            .ok_or("Player not found")?;

        if let Some(club) = &player.club {
            if club.eq_ignore_ascii_case(&new_club_name) {
                return Err(String::from("Player is already in this club!"));
            }
        }

        let new_club = self
            .clubs
            .iter_mut()
            .find(|club| club.name.eq_ignore_ascii_case(&new_club_name))
            .ok_or("New club not found")?;

        if new_club.transfer_budget < fee {
            return Err("Not enough money!")?;
        }

        new_club.transfer_budget -= fee;

        let current_club_name = player.club.as_deref().ok_or("Player must have a club")?;

        let current_club = self
            .clubs
            .iter_mut()
            .find(|club| club.name.eq_ignore_ascii_case(current_club_name))
            .ok_or("Current club not found")?;

        current_club.transfer_budget += fee;

        player.club = Some(new_club_name.clone());

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
