use crate::{
    board::Board,
    utils::{index_to_coords, is_valid_move},
};
use crate::{piece::PieceType::King, utils::coords_to_index};

pub fn get_king_moves(index: usize, board: &Board) -> Vec<usize> {
    let current_piece = &board.squares[index].piece;
    let (x, y) = index_to_coords(index);
    let possible_moves: [(i8, i8); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let mut v = Vec::new();

    match current_piece {
        Some(piece) => {
            if piece.piece_type != King {
                panic!("This piece is not a king");
            }

            for (dx, dy) in possible_moves {
                if is_valid_move(x as i8 + dx, y as i8 + dy, board, &piece.color) {
                    v.push(coords_to_index(x, y));
                }
            }
        }
        None => panic!("No piece at index"),
    }

    v
}
