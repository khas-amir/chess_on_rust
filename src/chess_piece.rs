use crate::constants::*;

pub enum Color {
    White,
    Black,
}

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub struct Piece {
    color: Color,
    piece_type: PieceType,
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
