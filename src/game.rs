use std::io;

use crate::club::Club;
use crate::file_reader::{create_clubs, create_players};
use crate::player::Player;

pub struct Game {
    pub clubs: Vec<Club>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn start(&mut self) -> () {
        loop {
            println!("Input command: ");

            let mut input: String = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: Vec<&str> = input.split(" - ").collect();

            if input.starts_with(&["info"]) {
                match self.get_info(&input) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }

            // ["transfer", "luka modric", "Liverpool", 40]
            if input.starts_with(&["transfer"]) {
                match self.transfer_player(&input) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
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

    pub fn get_info(&self, input: &Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
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

    pub fn transfer_player(
        &mut self,
        input: &Vec<&str>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if input.len() < 4 {
            panic!("Invalid args number");
        }

        let player_name = input[1].trim();
        let new_club_name = input[2].trim();
        let fee: u16 = input[3].trim().parse::<u16>().unwrap();

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

                if new_club.transfer_budget < fee {
                    panic!("Not enough money!")
                }

                new_club.buy_player(player, fee);

                println!(
                    "{} bought {} from {} for {} mil.",
                    new_club.name, player.name, current_club_name, fee
                );

                let current_club = self
                    .clubs
                    .iter_mut()
                    .find(|c| c.name.to_lowercase() == current_club_name.to_lowercase().trim())
                    .unwrap();

                current_club.sell_player(player, fee);

                Ok(true)
            }
            None => {
                panic!("Player not found, try again: ");
            }
        }
    }
}
