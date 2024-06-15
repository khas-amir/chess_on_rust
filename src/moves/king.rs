use crate::{
    board::Board,
    utils::{index_to_coords, is_valid_move},
};
use crate::{piece::PieceType::King, utils::coords_to_index};

pub fn get_king_moves(index: usize, board: &Board) -> Vec<usize> {
    let piece = match &board.squares[index].piece {
        Some(piece) => piece,
        None => panic!("No piece at index"),
    };
    if piece.piece_type != King {
        panic!("This piece is not a king");
    }
    let possible_moves: [(i8, i8); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut moves = Vec::new();
    let (x, y) = index_to_coords(index);
    for (dx, dy) in possible_moves {
        let new_x = x as i8 + dx;
        let new_y = y as i8 + dy;
        if is_valid_move(new_x, new_y, board, &piece.color) {
            moves.push(coords_to_index(new_x as usize, new_y as usize));
        }
    }
    moves
}
