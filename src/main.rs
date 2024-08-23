mod club;
mod country;
mod file_reader;
mod game;
mod player;
mod setup;

use std::io;

use game::Game;

fn main() -> Result<(), io::Error> {
    let mut game = Game::new();
    Ok(Game::start(&mut game)?)
}
