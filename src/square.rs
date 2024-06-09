use crate::chess_piece::Piece;
use std::fmt::{Display, Formatter};

pub struct Square {
    piece: Option<Piece>,
}

impl Square {
    pub fn new(piece: Option<Piece>) -> Self {
        Self { piece }
    }

    fn draw(&self) {
        match &self.piece {
            Some(piece) => print!(" {} |", piece),
            None => print!("   |"),
        }
    }
}

impl Display for Square {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        self.draw();
        Ok(())
    }
}
