use std::io::{self};

mod player;
use crate::player::{print_best_player_awards, Player};

mod file_reader;
use crate::file_reader::FileReader;

fn main() {
    let file_reader = FileReader {
        filename: "players.txt".to_string(),
    };

    let mut players: Vec<Player> = file_reader.create_players();

    let transfer_player = &mut players.first_mut().unwrap();
    transfer_player.transfer("Manchester United".to_string(), 50);
    println!("{}", transfer_player.player_info());

    println!("Please input player name: ");

    loop {
        let mut player_name_guess: String = String::new();

        io::stdin()
            .read_line(&mut player_name_guess)
            .expect("Failed to read line");

        match Player::find_player_by_name(&players, &player_name_guess) {
            Some(player) => {
                println!("{}", player.player_info());

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

                print_best_player_awards(&player.name);

                break;
            }
            None => println!("Player not found, try again: "),
        }
    }
}
