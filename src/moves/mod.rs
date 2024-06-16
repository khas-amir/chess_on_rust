pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::board::Board;
use crate::moves::bishop::get_bishop_moves;
use crate::moves::king::get_king_moves;
use crate::moves::knight::get_knight_moves;
use crate::moves::pawn::get_pawn_moves;
use crate::moves::queen::get_queen_moves;
use crate::moves::rook::get_rook_moves;
use crate::piece::PieceType::*;

pub fn get_moves(index: usize, board: &Board) -> Vec<usize> {
    let piece = match &board.squares[index].piece {
        Some(piece) => piece,
        None => panic!("No piece at index"),
    };

    match piece.piece_type {
        King => get_king_moves(index, board),
        Queen => get_queen_moves(index, board),
        Rook => get_rook_moves(index, board),
        Bishop => get_bishop_moves(index, board),
        Knight => get_knight_moves(index, board),
        Pawn => get_pawn_moves(index, board),
    }
}

#[cfg(test)]
mod tests;
