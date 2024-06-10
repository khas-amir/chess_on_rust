use crate::chess_piece::Piece;
use std::fmt::{Display, Formatter};

pub struct Square {
    piece: Option<Piece>,
}

impl Square {
    pub fn new(piece: Option<Piece>) -> Self {
        Self { piece }
    }

    pub fn draw(&self) -> String {
        match &self.piece {
            Some(piece) => format!("{piece}"),
            None => " ".to_string(),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.draw())
    }
}
