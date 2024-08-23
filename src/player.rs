use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub age: u8,
    pub position: Position,
    pub market_value: u8,
    pub club: Option<String>,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {} years - {:?} - {} mil - {}",
            self.name,
            self.age,
            self.position,
            self.market_value,
            self.club.as_deref().unwrap_or("Free agent")
        )
    }
}

#[derive(Debug, Clone)]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

impl Player {
    pub fn find_player_by_name<'a>(
        players: &'a Vec<Player>,
        input_name: &'a &str,
    ) -> Option<&'a Player> {
        players
            .iter()
            .find(|player| player.name.to_lowercase() == input_name.to_lowercase().trim())
    }

    pub fn get_text_file() -> &'static str {
        const PLAYERS_FILE: &str = "players.txt";
        PLAYERS_FILE
    }
}

pub enum PlayerParseError {
    InvalidFormat,
    InvalidAge,
    InvalidPosition,
    InvalidMarketValue,
}

impl Display for PlayerParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PlayerParseError::InvalidFormat => write!(f, "Invalid input format"),
            PlayerParseError::InvalidAge => write!(f, "Invalid age"),
            PlayerParseError::InvalidPosition => write!(f, "Invalid position"),
            PlayerParseError::InvalidMarketValue => write!(f, "Invalid market value"),
        }
    }
}

impl FromStr for Player {
    type Err = PlayerParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" - ").collect();

        if parts.len() != 4 {
            return Err(PlayerParseError::InvalidFormat);
        }

        let name = parts[0].trim().to_string();

        let age = parts[1]
            .trim()
            .parse::<u8>()
            .map_err(|_| PlayerParseError::InvalidAge)?;

        let position: Position = match parts[2].trim() {
            "GK" => Position::Goalkeeper,
            "CB" | "DL" | "DR" => Position::Defender,
            "CM" | "DM" => Position::Midfielder,
            "LW" | "RW" | "CF" | "ST" => Position::Forward,
            _ => return Err(PlayerParseError::InvalidPosition),
        };

        let market_value = parts[3]
            .trim()
            .parse::<u8>()
            .map_err(|_| PlayerParseError::InvalidMarketValue)?;

        Ok(Player {
            name,
            age,
            position,
            market_value,
            club: None,
        })
    }
}
