use crate::utils::{coords_to_index, index_to_coords, is_valid_move};
use crate::{board::Board, piece::PieceType::Pawn};

pub fn get_pawn_moves(index: usize, board: &Board) -> Vec<usize> {
    let piece = match &board.squares[index].piece {
        Some(piece) => piece,
        None => panic!("No piece at index"),
    };

    if piece.piece_type != Pawn {
        panic!("This piece is not a pawn");
    }

    let (x, y) = index_to_coords(index);
    let mut moves = Vec::new();

    let possible_moves: Vec<(i8, i8)> = if y == 1 {
        vec![(0, 1), (0, 2)]
    } else {
        vec![(0, 1)]
    };

    let moves_to_enemy: Vec<(i8, i8)> = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];

    for (dx, dy) in possible_moves {
        let new_x = x as i8 + dx;
        let new_y = y as i8 + dy;

        if !is_valid_move(new_x, new_y, board, &piece.color) {
            break;
        }

        let new_index = coords_to_index(new_x as usize, new_y as usize);
        if board.squares[new_index].piece.is_some() {
            break;
        }

        moves.push(new_index);
    }

    for (dx, dy) in moves_to_enemy {
        let new_x = x as i8 + dx;
        let new_y = y as i8 + dy;

        if is_valid_move(new_x, new_y, board, &piece.color) {
            let new_index = coords_to_index(new_x as usize, new_y as usize);

            if let Some(maybe_enemy) = &board.squares[new_index].piece {
                if piece.color != maybe_enemy.color {
                    moves.push(new_index);
                }
            }
        }
    }
    moves
}
