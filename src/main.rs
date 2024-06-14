mod board;
mod constants;
mod game;
mod moves;
mod piece;
mod square;
mod utils;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.start();
}
