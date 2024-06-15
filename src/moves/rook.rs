use crate::board::Board;
use crate::piece::PieceType::Rook;
use crate::utils::coords_to_index;
use crate::utils::get_horizontal_moves;
use crate::utils::get_vertical_moves;
use crate::utils::index_to_coords;
use crate::utils::is_valid_move;

pub fn get_rook_moves(index: usize, board: &Board) -> Vec<usize> {
    let current_piece = match &board.squares[index].piece {
        Some(piece) => piece,
        None => panic!("No piece at index"),
    };

    if current_piece.piece_type != Rook {
        panic!("This piece is not a rook");
    }

    let (x, y) = index_to_coords(index);
    let directions = vec![
        get_horizontal_moves(x, y, true),
        get_horizontal_moves(x, y, false),
        get_vertical_moves(x, y, true),
        get_vertical_moves(x, y, false),
    ];

    let mut moves = Vec::new();

    for direction in directions {
        for (new_x, new_y) in direction {
            if !is_valid_move(new_x, new_y, board, &current_piece.color) {
                break;
            }
            let new_index = coords_to_index(new_x as usize, new_y as usize);
            moves.push(new_index);
            if board.squares[new_index].piece.is_some() {
                break;
            }
        }
    }
    moves
}
