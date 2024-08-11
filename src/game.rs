use crate::club::Club;
use crate::file_reader::get_file_content;
use crate::player::{Player, Position};

pub struct Game {
    pub clubs: Vec<Club>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(filename: &str) -> Game {
        let mut clubs: Vec<Club> = Vec::new();
        let players: Vec<Player> = Vec::new();

        for line in get_file_content(filename).lines() {
            match Game::parse_line(line) {
                Some(value) => {
                    let club_name = value.0;
                    let player = value.1;

                    let club = clubs.iter_mut().find(|club| club.name == club_name);
                    match club {
                        Some(club) => club.squad.push(player),
                        None => {
                            let mut new_club = Club::new(club_name);
                            new_club.squad.push(player);
                        }
                    }
                }
                None => (),
            };
        }

        Game { clubs, players }
    }

    fn parse_line(line: &str) -> Option<(String, Player)> {
        let parts: Vec<&str> = line.split(" - ").collect();

        if parts.len() != 5 {
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

        let club = parts[3].trim().to_string();

        let market_value = parts[4].trim().parse::<u8>();
        let market_value: u8 = match market_value {
            Ok(value) => value,
            Err(_) => 0,
        };

        Some((
            club,
            Player {
                name,
                age,
                position,
                market_value,
            },
        ))
    }
}
