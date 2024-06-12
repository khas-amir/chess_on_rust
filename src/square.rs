use crate::piece::Piece;
use std::fmt::{Display, Formatter};

pub struct Square {
    pub piece: Option<Piece>,
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::piece::get_black_piece;
    use crate::piece::PieceType::*;

    #[test]
    fn test_display() {
        let square = Square { piece: None };
        assert_eq!(square.draw(), " ");

        let square = Square {
            piece: Some(get_black_piece(Pawn)),
        };
        assert_eq!(square.draw(), "♟");
    }
}
