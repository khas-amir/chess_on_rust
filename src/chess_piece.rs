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

fn get_symbol(color: Color, piece: PieceType) -> char {
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

pub fn get_white_piece(piece: PieceType) -> char {
    get_symbol(Color::White, piece)
}

pub fn get_black_piece(piece: PieceType) -> char {
    get_symbol(Color::Black, piece)
}
