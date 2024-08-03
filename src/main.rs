use std::fs::read_to_string;
use std::io::{self};

use crate::player::Player;
pub mod player;

// TODO Extract modules
fn main() {
    const FILE_NAME: &str = "players.txt";
    // TODO error handling
    let players: Vec<Player> = read_to_string(FILE_NAME)
        .unwrap()
        .lines()
        .filter_map(Player::create_from_line)
        .collect();

    println!("Please input player name: ");

    loop {
        let mut player_name_guess: String = String::new();

        io::stdin()
            .read_line(&mut player_name_guess)
            .expect("Failed to read line");

        match Player::find_player_by_name(&players, &player_name_guess) {
            Some(player) => {
                if Player::is_oldest(&players, player) {
                    println!(
                        "{} is the oldest player in the squad - {} years old.",
                        player.name, player.age
                    );
                }

                if Player::is_most_valued(&players, player) {
                    println!(
                        "{} is the most valued player in the squad - {} million.",
                        player.name, player.market_value
                    );
                }

                println!("{}", player.player_info());
                break;
            }
            None => println!("Player not found, try again: "),
        }
    }
}
