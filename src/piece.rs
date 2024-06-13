use crate::constants::*;

#[derive(PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(PartialEq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    icon: char,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        let icon = get_symbol(&color, &piece_type);
        Self {
            color,
            piece_type,
            icon,
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.icon)
    }
}

fn get_symbol(color: &Color, piece: &PieceType) -> char {
    match (color, piece) {
        (Color::White, PieceType::King) => KING_WHITE,
        (Color::Black, PieceType::King) => KING_BLACK,
        (Color::White, PieceType::Queen) => QUEEN_WHITE,
        (Color::Black, PieceType::Queen) => QUEEN_BLACK,
        (Color::White, PieceType::Rook) => ROOK_WHITE,
        (Color::Black, PieceType::Rook) => ROOK_BLACK,
        (Color::White, PieceType::Bishop) => BISHOP_WHITE,
        (Color::Black, PieceType::Bishop) => BISHOP_BLACK,
        (Color::White, PieceType::Knight) => KNIGHT_WHITE,
        (Color::Black, PieceType::Knight) => KNIGHT_BLACK,
        (Color::White, PieceType::Pawn) => PAWN_WHITE,
        (Color::Black, PieceType::Pawn) => PAWN_BLACK,
    }
}

pub fn get_white_piece(piece: PieceType) -> Piece {
    Piece::new(Color::White, piece)
}

pub fn get_black_piece(piece: PieceType) -> Piece {
    Piece::new(Color::Black, piece)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_symbol() {
        assert_eq!(get_symbol(&Color::White, &PieceType::King), KING_WHITE);
        assert_eq!(get_symbol(&Color::Black, &PieceType::King), KING_BLACK);
        assert_eq!(get_symbol(&Color::White, &PieceType::Queen), QUEEN_WHITE);
    }

    #[test]
    fn piece_creation() {
        let piece = Piece::new(Color::White, PieceType::King);
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.piece_type, PieceType::King);

        let piece = Piece::new(Color::Black, PieceType::Queen);
        assert_eq!(piece.color, Color::Black);
        assert_eq!(piece.piece_type, PieceType::Queen);
    }

    #[test]
    fn piece_display() {
        let piece = Piece::new(Color::White, PieceType::King);
        let output = format!("{}", piece);
        assert_eq!(output, KING_WHITE.to_string());

        let piece = Piece::new(Color::Black, PieceType::Pawn);
        let output = format!("{}", piece);
        assert_eq!(output, PAWN_BLACK.to_string());
    }
}
