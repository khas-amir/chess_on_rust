use crate::board::Board;
use crate::piece::PieceType;
use crate::utils::coords_to_index;
use crate::utils::get_diagonal_moves;
use crate::utils::index_to_coords;
use crate::utils::is_valid_move;

pub fn get_bishop_moves(index: usize, board: &Board) -> Vec<usize> {
    let piece = match &board.squares[index].piece {
        Some(piece) => piece,
        None => panic!("No piece at index"),
    };

    if (piece.piece_type != PieceType::Bishop) {
        panic!("Piece is not a bishop");
    }

    let mut moves: Vec<usize> = vec![];

    let (x, y) = index_to_coords(index);

    let directions = [
        get_diagonal_moves(x, y, true, true),
        get_diagonal_moves(x, y, true, false),
        get_diagonal_moves(x, y, false, true),
        get_diagonal_moves(x, y, false, false),
    ];

    for direction in directions {
        for (new_x, new_y) in direction {
            if !is_valid_move(new_x, new_y, board, &piece.color) {
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
