use crate::chess_piece::{get_black_piece, get_white_piece, PieceType};
use crate::square::Square;
use std::fmt::{Display, Formatter};

pub struct Board {
    squares: Vec<Square>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: (0..64).map(Self::map_pieces).collect(),
        }
    }

    fn map_pieces(index: u8) -> Square {
        let x = index % 8;
        let y = index / 8;
        let piece = match (x, y) {
            // Белые фигуры
            (0, 7) | (7, 7) => Some(get_white_piece(PieceType::Rook)), // Ладьи
            (1, 7) | (6, 7) => Some(get_white_piece(PieceType::Knight)), // Кони
            (2, 7) | (5, 7) => Some(get_white_piece(PieceType::Bishop)), // Слоны
            (4, 7) => Some(get_white_piece(PieceType::King)),          // Король
            (_, 6) => Some(get_white_piece(PieceType::Pawn)),          // Пешки
            (3, 7) => Some(get_white_piece(PieceType::Queen)),         // Королева

            // Чёрные фигуры
            (0, 0) | (7, 0) => Some(get_black_piece(PieceType::Rook)), // Ладьи
            (1, 0) | (6, 0) => Some(get_black_piece(PieceType::Knight)), // Кони
            (2, 0) | (5, 0) => Some(get_black_piece(PieceType::Bishop)), // Слоны
            (3, 0) => Some(get_black_piece(PieceType::Queen)),         // Королева
            (4, 0) => Some(get_black_piece(PieceType::King)),          // Король
            (_, 1) => Some(get_black_piece(PieceType::Pawn)),          // Пешки
            _ => None,
        };
        Square::new(piece)
    }

    fn draw(&self) {
        println!("    A   B   C   D   E   F   G   H");
        for i in 0..8 {
            println!("  +---+---+---+---+---+---+---+---+");
            print!("{} |", 8 - i);
            for j in 0..8 {
                let index = i * 8 + j;
                print!("{}", self.squares[index]);
            }
            print!(" {}", 8 - i);
            println!();
        }
        println!("  +---+---+---+---+---+---+---+---+");
        println!("    A   B   C   D   E   F   G   H");
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        let from_index = from.0 * 8 + from.1;
        let to_index = to.0 * 8 + to.1;
        self.squares.swap(from_index, to_index);
    }
}

impl Display for Board {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        self.draw();
        Ok(())
    }
}
