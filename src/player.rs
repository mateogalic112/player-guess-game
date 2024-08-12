use std::collections::HashMap;

use crate::{club::Club, game::Game};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub age: u8,
    pub position: Position,
    pub market_value: u8,
}

#[derive(Debug)]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

pub fn print_best_player_awards(player_name: &str) {
    let mut best_players: HashMap<String, Vec<i16>> = HashMap::new();
    best_players.insert(String::from("Cristiano Ronaldo"), vec![2008, 2013]);
    best_players.insert(String::from("Falcao"), vec![2012]);

    if let Some(years) = best_players.get(player_name) {
        let years: String = years
            .iter()
            .map(|year| year.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        println!("{player_name} was the best player in the world in: {years}");
    }
}

impl Player {
    pub fn player_info(&self, club: &Club) -> String {
        format!(
            "Player {} ({}, {:?}) plays for {}. Valued at: {} mil.",
            self.name, self.age, self.position, club.name, self.market_value
        )
    }

    pub fn is_older(&self, other: &Player) -> bool {
        self.age > other.age
    }

    pub fn is_more_valued(&self, other: &Player) -> bool {
        self.market_value > other.market_value
    }

    pub fn find_club<'a>(&'a self, game: &'a Game) -> &Club {
        let club = &game
            .clubs
            .iter()
            .find(|club| club.squad.iter().any(|p| p.name == self.name))
            .unwrap();

        club
    }

    pub fn find_player_by_name<'a>(
        players: &'a Vec<Player>,
        input_name: &'a String,
    ) -> Option<&'a Player> {
        players
            .iter()
            .find(|player| player.name.to_lowercase() == input_name.to_lowercase().trim())
    }

    pub fn is_oldest(players: &Vec<Player>, player: &Player) -> bool {
        players
            .iter()
            .all(|other| other.name == player.name || player.is_older(other))
    }

    pub fn is_most_valued(players: &Vec<Player>, player: &Player) -> bool {
        players
            .iter()
            .all(|other| other.name == player.name || player.is_more_valued(other))
    }
}

impl Player {
    pub fn new(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(" - ").collect();

        if parts.len() != 4 {
            return None;
        }

        let name = parts[0].trim().to_string();

        let age = parts[1].trim().parse::<u8>();
        let age = match age {
            Ok(value) => value,
            Err(_) => 0,
        };

        let position: Position = match parts[2].trim() {
            "GK" => Position::Goalkeeper,
            "CB" | "DL" | "DR" => Position::Defender,
            "CM" | "DM" => Position::Midfielder,
            "CF" | "ST" => Position::Forward,
            _ => return None,
        };

        let market_value = parts[3].trim().parse::<u8>();
        let market_value: u8 = match market_value {
            Ok(value) => value,
            Err(_) => 0,
        };

        Some(Player {
            name,
            age,
            position,
            market_value,
        })
    }

    pub fn get_text_file() -> &'static str {
        const PLAYERS_FILE: &str = "players.txt";
        PLAYERS_FILE
    }
}
