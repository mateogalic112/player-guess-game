use crate::club::Club;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub age: u8,
    pub position: Position,
    pub market_value: u8,
}

#[derive(Debug, Clone)]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

impl Player {
    pub fn player_info(&self, club: &Club) -> String {
        format!(
            "Player {} ({}, {:?}) plays for {}. Valued at: {} mil.",
            self.name, self.age, self.position, club.name, self.market_value
        )
    }

    pub fn find_club<'a>(&'a self, clubs: &'a Vec<Club>) -> &Club {
        let club = &clubs
            .iter()
            .find(|club| club.squad.iter().any(|p| p.name == self.name))
            .unwrap();

        club
    }

    pub fn find_player_by_name<'a>(
        players: &'a Vec<Player>,
        input_name: &'a &str,
    ) -> Option<&'a Player> {
        players
            .iter()
            .find(|player| player.name.to_lowercase() == input_name.to_lowercase().trim())
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
            "LW" | "RW" | "CF" | "ST" => Position::Forward,
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
