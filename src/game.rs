use crate::board::Board;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
        }
    }

    pub fn start(&mut self) {
        // example fo moving
        self.board.move_piece((0, 0), (3, 0));
        println!("{}", self.board);
    }
}
