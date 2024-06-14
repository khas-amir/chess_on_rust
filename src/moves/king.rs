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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_king_moves() {
        let mut board = Board::new();
        let possible_moves = get_king_moves(4, &board);

        assert_eq!(possible_moves.len(), 0);

        board.move_piece((0, 4), (2, 2));
        let index = 2 * 8 + 2;
        let possible_moves = get_king_moves(index, &board);
        assert_eq!(possible_moves.len(), 5);

        board.move_piece((2, 2), (3, 2));
        let index = 3 * 8 + 2;
        let possible_moves = get_king_moves(index, &board);
        assert_eq!(possible_moves.len(), 8);

        board.move_piece((3, 2), (4, 2));
        let index = 4 * 8 + 2;
        let possible_moves = get_king_moves(index, &board);
        assert_eq!(possible_moves.len(), 8);

        board.move_piece((4, 2), (5, 2));
        let index = 5 * 8 + 2;
        let possible_moves = get_king_moves(index, &board);
        assert_eq!(possible_moves.len(), 8);
    }

    #[test]
    #[should_panic = "This piece is not a king"]
    fn test_get_king_moves_invalid() {
        let board = Board::new();
        get_king_moves(2, &board);
        get_king_moves(5, &board);
        get_king_moves(11, &board);
    }

    #[test]
    #[should_panic = "No piece at index"]
    fn test_get_king_moves_empty() {
        let board = Board::new();
        let index = 2 * 8 + 2;
        get_king_moves(index, &board);
        let index = 5 * 8 + 2;
        get_king_moves(index, &board);
        let index = 11 * 8 + 2;
        get_king_moves(index, &board);
    }
}