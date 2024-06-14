use crate::utils::{coords_to_index, index_to_coords, is_valid_move};
use crate::{board::Board, piece::PieceType::Pawn};

pub fn get_pawn_moves(index: usize, board: &Board) -> Vec<usize> {
    let current_piece = &board.squares[index].piece;
    let (x, y) = index_to_coords(index);
    let mut v = Vec::new();
    let possible_moves: Vec<(i8, i8)> = if y == 1 {
        vec![(0, 1), (0, 2)]
    } else {
        vec![(0, 1)]
    };

    let moves_to_enemy: Vec<(i8, i8)> = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];

    match current_piece {
        Some(current) => {
            if current.piece_type != Pawn {
                panic!("This piece is not a pawn");
            }

            for (dx, dy) in possible_moves {
                let new_x = x as i8 + dx;
                let new_y = y as i8 + dy;

                if !is_valid_move(new_x, new_y, board, &current.color) {
                    break;
                }

                let new_index = coords_to_index(new_x as usize, new_y as usize);
                if board.squares[new_index].piece.is_some() {
                    break;
                }

                v.push(new_index);
            }
            for (dx, dy) in moves_to_enemy {
                let new_x = x as i8 + dx;
                let new_y = y as i8 + dy;

                if is_valid_move(new_x, new_y, board, &current.color) {
                    let new_index = coords_to_index(new_x as usize, new_y as usize);

                    if let Some(piece) = &board.squares[new_index].piece {
                        if current.color != piece.color {
                            v.push(new_index);
                        }
                    }
                }
            }
        }
        None => panic!("No piece at index"),
    }
    v
}
