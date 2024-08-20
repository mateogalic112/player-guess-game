use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::country::Country;
use crate::player::Player;

#[derive(Clone)]
pub struct Club {
    pub country: Country,
    pub name: String,
    pub transfer_budget: u16,
    pub squad: Vec<Player>,
}

impl Display for Club {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub enum ClubParseError {
    InvalidFormat,
    UnknownCountry,
    InvalidBudget,
}

impl Display for ClubParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClubParseError::InvalidFormat => write!(f, "Invalid input format"),
            ClubParseError::UnknownCountry => write!(f, "Unknown country"),
            ClubParseError::InvalidBudget => write!(f, "Invalid transfer budget"),
        }
    }
}

impl FromStr for Club {
    type Err = ClubParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" - ").collect();

        if parts.len() != 3 {
            return Err(ClubParseError::InvalidFormat);
        }

        let country: Country = match parts[0].trim() {
            "England" => Country::England,
            "Spain" => Country::Spain,
            _ => return Err(ClubParseError::UnknownCountry),
        };

        let name = parts[1].trim().to_string();

        let transfer_budget = parts[2].trim().parse::<u16>();
        let transfer_budget: u16 = match transfer_budget {
            Ok(value) => value,
            Err(_) => return Err(ClubParseError::InvalidBudget),
        };

        Ok(Club {
            country,
            name,
            transfer_budget,
            squad: Vec::new(),
        })
    }
}

impl Club {
    pub fn sell_player(&mut self, player: &Player, fee: u16) -> () {
        self.squad.retain(|p| p.name != player.name);
        self.transfer_budget += fee;
    }

    pub fn buy_player(&mut self, player: &Player, fee: u16) -> () {
        self.squad.push(player.clone());
        self.transfer_budget -= fee;
    }

    pub fn get_text_file() -> &'static str {
        const CLUBS_FILE: &str = "clubs.txt";
        CLUBS_FILE
    }
}
