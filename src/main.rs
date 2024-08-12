use std::{env, io};

mod player;
use crate::player::Player;

mod file_reader;

mod game;
use game::Game;

mod club;

fn main() {
    let game: Game = Game::new();

    let args: Vec<String> = env::args().collect();

    println!("Please input player name: ");

    loop {
        let mut player_name_guess: String = String::new();

        io::stdin()
            .read_line(&mut player_name_guess)
            .expect("Failed to read line");

        match Player::find_player_by_name(&game.players, &player_name_guess) {
            Some(player) => {
                println!("{}", player.player_info(player.find_club(&game)));
            }
            None => println!("Player not found, try again: "),
        }
    }
}
