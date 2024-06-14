use crate::{board::Board, piece::Color};

pub fn index_to_coords(index: usize) -> (usize, usize) {
    (index % 8, index / 8)
}

pub fn coords_to_index(x: usize, y: usize) -> usize {
    y * 8 + x
}

pub fn is_valid_move(new_x: i8, new_y: i8, board: &Board, color: &Color) -> bool {
    if !(0..8).contains(&new_x) || !(0..8).contains(&new_y) {
        return false;
    }

    let possible_square = &board.squares[coords_to_index(new_x as usize, new_y as usize)];
    match &possible_square.piece {
        Some(piece) => &piece.color != color,
        None => true,
    }
}
