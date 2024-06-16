use chess::game::Game;
use chess::input::Input;

fn main() {
    let mut game = Game::new();
    println!("\n{}", game.get_board());
    let (from, to) = Input::input_from_to_coord_str();
    game.make_move(from, to);
    println!("\n{}", game.get_board());
}
