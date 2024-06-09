mod board;
mod chess_piece;
mod constants;
mod game;
mod square;
use game::Game;

fn main() {
    let game = Game::new();
    game.start();
}
