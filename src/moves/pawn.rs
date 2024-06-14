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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::piece::{get_black_piece, get_white_piece};

    #[test]
    fn test_pawn_initial_double_move() {
        let mut board = Board::new();
        board.squares[8].piece = Some(get_black_piece(Pawn));
        let moves = get_pawn_moves(8, &board);
        assert_eq!(moves, vec![16, 24]);
    }

    #[test]
    fn test_pawn_single_move() {
        let mut board = Board::new();
        board.squares[16].piece = Some(get_black_piece(Pawn));
        let moves = get_pawn_moves(16, &board);
        assert_eq!(moves, vec![24]);
    }

    #[test]
    fn test_pawn_blocked_by_ally() {
        let mut board = Board::new();
        board.squares[8].piece = Some(get_black_piece(Pawn));
        board.squares[16].piece = Some(get_black_piece(Pawn));
        let moves = get_pawn_moves(8, &board);
        assert_eq!(moves, vec![]);
    }

    #[test]
    fn test_pawn_blocked_by_enemy() {
        let mut board = Board::new();
        board.squares[8].piece = Some(get_black_piece(Pawn));
        board.squares[16].piece = Some(get_white_piece(Pawn));
        let moves = get_pawn_moves(8, &board);
        assert_eq!(moves, vec![]);
    }

    #[test]
    fn test_pawn_initial_double_move_blocked() {
        let mut board = Board::new();
        board.squares[8].piece = Some(get_black_piece(Pawn));
        board.squares[16].piece = Some(get_black_piece(Pawn));
        let moves = get_pawn_moves(8, &board);
        assert_eq!(moves, vec![]);
    }

    #[test]
    fn test_pawn_move_to_enemy_occupied_square() {
        let mut board = Board::new();
        board.squares[3 * 8 + 3].piece = Some(get_white_piece(Pawn));
        board.squares[4 * 8 + 4].piece = Some(get_black_piece(Pawn));
        let moves = get_pawn_moves(4 * 8 + 4, &board);
        assert_eq!(moves, vec![44, 3 * 8 + 3]);
    }
}
