use std::fs::read_to_string;
use std::io::{self};

mod player;
use crate::player::Player;

fn main() {
    const FILE_NAME: &str = "players.txt";
    let mut players: Vec<Player> = read_to_string(FILE_NAME)
        .unwrap()
        .lines()
        .filter_map(Player::create_from_line)
        .collect();

    println!("Please input player name: ");

    let transfer_player = &mut players.first_mut().unwrap();
    transfer_player.transfer("Manchester United".to_string(), 50);
    println!("{}", transfer_player.player_info());

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
