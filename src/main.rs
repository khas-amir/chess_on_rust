mod board;
mod constants;
mod game;
mod move_logic;
mod piece;
mod square;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.start();
}
