use crate::piece::{get_black_piece, get_white_piece, PieceType};
use crate::square::Square;
use std::fmt::{Display, Formatter};

pub struct Board {
    pub squares: Vec<Square>,
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

    pub fn draw(&self) -> String {
        let mut result = String::new();
        result += "    A   B   C   D   E   F   G   H\n";
        for i in 0..8 {
            result += "  +---+---+---+---+---+---+---+---+\n";
            let number = 8 - i;
            result += format!("{} |", number).as_str();
            for j in 0..8 {
                let index = i * 8 + j;
                let square_str = self.squares[index].to_string();
                let square_str = format!(" {} |", square_str);
                result += &square_str;
            }
            result += format!(" {}", number).as_str();
            result.push('\n')
        }
        result += "  +---+---+---+---+---+---+---+---+\n";
        result += "    A   B   C   D   E   F   G   H\n";
        result
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        let from_index = from.0 * 8 + from.1;
        let to_index = to.0 * 8 + to.1;
        self.squares.swap(from_index, to_index);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.draw())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_board() {
        let expected = String::from(
            "    A   B   C   D   E   F   G   H
  +---+---+---+---+---+---+---+---+
8 | ♜ | ♞ | ♝ | ♛ | ♚ | ♝ | ♞ | ♜ | 8
  +---+---+---+---+---+---+---+---+
7 | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | 7
  +---+---+---+---+---+---+---+---+
6 |   |   |   |   |   |   |   |   | 6
  +---+---+---+---+---+---+---+---+
5 |   |   |   |   |   |   |   |   | 5
  +---+---+---+---+---+---+---+---+
4 |   |   |   |   |   |   |   |   | 4
  +---+---+---+---+---+---+---+---+
3 |   |   |   |   |   |   |   |   | 3
  +---+---+---+---+---+---+---+---+
2 | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | 2
  +---+---+---+---+---+---+---+---+
1 | ♖ | ♘ | ♗ | ♕ | ♔ | ♗ | ♘ | ♖ | 1
  +---+---+---+---+---+---+---+---+
    A   B   C   D   E   F   G   H
",
        );
        let board = Board::new();
        let board_str = board.draw();
        assert_eq!(board_str, expected);
    }

    #[test]
    fn test_moving() {
        let mut board = Board::new();
        let x_from = 5;
        let y_from = 1;
        let x_to = 3;
        let y_to = 2;

        let index_from = y_from * 8 + x_from;
        let index_to = y_to * 8 + x_to;

        board.move_piece((y_from, x_from), (y_to, x_to));

        let expected = String::from(
            "    A   B   C   D   E   F   G   H
  +---+---+---+---+---+---+---+---+
8 | ♜ | ♞ | ♝ | ♛ | ♚ | ♝ | ♞ | ♜ | 8
  +---+---+---+---+---+---+---+---+
7 | ♟ | ♟ | ♟ | ♟ | ♟ |   | ♟ | ♟ | 7
  +---+---+---+---+---+---+---+---+
6 |   |   |   | ♟ |   |   |   |   | 6
  +---+---+---+---+---+---+---+---+
5 |   |   |   |   |   |   |   |   | 5
  +---+---+---+---+---+---+---+---+
4 |   |   |   |   |   |   |   |   | 4
  +---+---+---+---+---+---+---+---+
3 |   |   |   |   |   |   |   |   | 3
  +---+---+---+---+---+---+---+---+
2 | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | 2
  +---+---+---+---+---+---+---+---+
1 | ♖ | ♘ | ♗ | ♕ | ♔ | ♗ | ♘ | ♖ | 1
  +---+---+---+---+---+---+---+---+
    A   B   C   D   E   F   G   H
",
        );
        assert_eq!(board.draw(), expected);
        assert!(board.squares[index_from].piece.is_none());
        assert!(board.squares[index_to].piece.is_some());

        board.move_piece((y_to, x_to), (y_from, x_from));
        let expected = String::from(
            "    A   B   C   D   E   F   G   H
  +---+---+---+---+---+---+---+---+
8 | ♜ | ♞ | ♝ | ♛ | ♚ | ♝ | ♞ | ♜ | 8
  +---+---+---+---+---+---+---+---+
7 | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | 7
  +---+---+---+---+---+---+---+---+
6 |   |   |   |   |   |   |   |   | 6
  +---+---+---+---+---+---+---+---+
5 |   |   |   |   |   |   |   |   | 5
  +---+---+---+---+---+---+---+---+
4 |   |   |   |   |   |   |   |   | 4
  +---+---+---+---+---+---+---+---+
3 |   |   |   |   |   |   |   |   | 3
  +---+---+---+---+---+---+---+---+
2 | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | 2
  +---+---+---+---+---+---+---+---+
1 | ♖ | ♘ | ♗ | ♕ | ♔ | ♗ | ♘ | ♖ | 1
  +---+---+---+---+---+---+---+---+
    A   B   C   D   E   F   G   H
",
        );
        assert_eq!(board.draw(), expected);
        assert!(board.squares[index_from].piece.is_some());
        assert!(board.squares[index_to].piece.is_none());
    }
}
