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

    pub fn start(&self) {
        println!("{}", self.board);
    }
}
