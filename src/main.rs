mod club;
mod country;
mod file_reader;
mod game;
mod player;
mod setup;

use game::Game;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();
    Ok(Game::start(&mut game)?)
}
