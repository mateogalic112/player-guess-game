mod club;
mod country;
mod file_reader;
mod game;
mod player;
mod setup;

use std::io::Result;

use game::Game;

fn main() -> Result<()> {
    let mut game = Game::new();
    Ok(Game::start(&mut game)?)
}
