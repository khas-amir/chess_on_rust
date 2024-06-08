mod board;
mod chess_piece;
mod constants;
mod game;

use game::Game;

fn main() {
    let game = Game::new();
    game.start();
}
