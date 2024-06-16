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

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.board.move_piece(from, to);
    }

    pub fn get_board(&self) -> String {
        self.board.to_string()
    }
}
