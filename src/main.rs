mod club;
mod file_reader;
mod game;
mod player;

use game::Game;

fn main() {
    let mut game = Game::new();
    Game::start(&mut game);
}
