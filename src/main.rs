use std::process;
use std::{env, io};

mod player;
use crate::player::{print_best_player_awards, Player};

mod file_reader;

mod shortlist;
use crate::shortlist::Shortlist;

mod game;
use game::Game;

mod club;

fn main() {
    let game: Game = Game::new();

    let args: Vec<String> = env::args().collect();

    let shortlist = Shortlist::build(&args).unwrap_or_else(|error| {
        println!("Problem with parsing args: {error}");
        process::exit(1);
    });
    println!("Query: {}", shortlist.query);

    println!("Please input player name: ");

    loop {
        let mut player_name_guess: String = String::new();

        io::stdin()
            .read_line(&mut player_name_guess)
            .expect("Failed to read line");

        match Player::find_player_by_name(&game.players, &player_name_guess) {
            Some(player) => {
                println!("{}", player.player_info(player.find_club(&game)));

                if Player::is_oldest(&game.players, player) {
                    println!(
                        "{} is the oldest player in the squad - {} years old.",
                        player.name, player.age
                    );
                }
                if Player::is_most_valued(&game.players, player) {
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
