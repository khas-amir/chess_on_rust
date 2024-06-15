#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        moves::queen::get_queen_moves,
        piece::{get_black_piece, get_white_piece, PieceType::Queen},
    };

    #[test]
    fn test_queen_initial_move() {
        let board = Board::new();
        let moves = get_queen_moves(3, &board);
        assert_eq!(moves, vec![]);
    }

    #[test]
    #[should_panic = "No piece at index"]
    fn get_queen_moves_panics_no_piece() {
        let board = Board::empty();
        get_queen_moves(32, &board);
    }

    #[test]
    #[should_panic = "There is no queen"]
    fn get_queen_moves_panics_no_queen() {
        let board = Board::new();
        get_queen_moves(2, &board);
    }

    #[test]
    fn get_queen_moves_from_top_left_edge() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_black_piece(Queen));
        let moves = get_queen_moves(0, &board);
        let expected = vec![
            1, 2, 3, 4, 5, 6, 7, // horizontal
            8, 16, 24, 32, 40, 48, 56, // vertical
            9, 18, 27, 36, 45, 54, 63, // diagonal
        ];
        assert_eq!(moves, expected);
    }

    #[test]
    fn get_queen_moves_from_top_right_edge() {
        let mut board = Board::empty();
        board.squares[7].piece = Some(get_black_piece(Queen));
        let moves = get_queen_moves(7, &board);
        let expected = vec![
            6, 5, 4, 3, 2, 1, 0, // horizontal
            15, 23, 31, 39, 47, 55, 63, // vertical
            14, 21, 28, 35, 42, 49, 56, // diagonal
        ];
        assert_eq!(moves, expected);
    }

    #[test]
    fn get_queen_moves_from_bottom_left_edge() {
        let mut board = Board::empty();
        board.squares[56].piece = Some(get_black_piece(Queen));
        let moves = get_queen_moves(56, &board);
        let expected = vec![
            57, 58, 59, 60, 61, 62, 63, // horizontal
            48, 40, 32, 24, 16, 8, 0, // vertical
            49, 42, 35, 28, 21, 14, 7, // diagonal
        ];
        assert_eq!(moves, expected);
    }

    #[test]
    fn get_queen_moves_in_the_middle_of_board() {
        let mut board = Board::empty();
        board.squares[27].piece = Some(get_black_piece(Queen));
        let moves = get_queen_moves(27, &board);
        let expected = vec![
            28, 29, 30, 31, // horizontal right
            26, 25, 24, // horizontal left
            35, 43, 51, 59, // vertical down
            19, 11, 3, // vertical up
            36, 45, 54, 63, // diagonal to bottom right
            20, 13, 6, // diagonal to top right
            34, 41, 48, // diagonal to bottom left
            18, 9, 0, // diagonal to top left
        ];
        assert_eq!(moves, expected);
    }

    #[test]
    fn get_queen_moves_in_the_middle_of_board_with_blocked_pieces() {
        let mut board = Board::empty();
        board.squares[27].piece = Some(get_black_piece(Queen));
        board.squares[28].piece = Some(get_black_piece(Queen));
        let moves = get_queen_moves(27, &board);
        let expected = vec![
            26, 25, 24, // horizontal left
            35, 43, 51, 59, // vertical down
            19, 11, 3, // vertical up
            36, 45, 54, 63, // diagonal to bottom right
            20, 13, 6, // diagonal to top right
            34, 41, 48, // diagonal to bottom left
            18, 9, 0, // diagonal to top left
        ];
        assert_eq!(moves, expected);
    }

    #[test]
    fn get_queen_moves_in_the_middle_of_board_with_blocked_pieces_and_captures() {
        let mut board = Board::empty();
        board.squares[27].piece = Some(get_black_piece(Queen));
        board.squares[28].piece = Some(get_black_piece(Queen));
        board.squares[25].piece = Some(get_white_piece(Queen));
        let moves = get_queen_moves(27, &board);
        let expected = vec![
            26, 25, // horizontal left
            35, 43, 51, 59, // vertical down
            19, 11, 3, // vertical up
            36, 45, 54, 63, // diagonal to bottom right
            20, 13, 6, // diagonal to top right
            34, 41, 48, // diagonal to bottom left
            18, 9, 0, // diagonal to top left
        ];
        assert_eq!(moves, expected);
    }

    #[test]
    fn get_queen_moves_when_surround_by_enemies() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_black_piece(Queen));
        board.squares[1].piece = Some(get_white_piece(Queen));
        board.squares[8].piece = Some(get_white_piece(Queen));
        board.squares[9].piece = Some(get_white_piece(Queen));
        let moves = get_queen_moves(0, &board);
        let expected = vec![1, 8, 9];
        assert_eq!(moves, expected);
    }

    #[test]
    fn get_queen_moves_when_surround_by_enemies2() {
        let mut board = Board::empty();
        board.squares[27].piece = Some(get_black_piece(Queen));
        board.squares[9].piece = Some(get_white_piece(Queen));
        board.squares[41].piece = Some(get_white_piece(Queen));
        board.squares[13].piece = Some(get_white_piece(Queen));
        board.squares[11].piece = Some(get_white_piece(Queen));
        board.squares[35].piece = Some(get_white_piece(Queen));
        let moves = get_queen_moves(27, &board);
        let expected = vec![
            28, 29, 30, 31, // horizontal right
            26, 25, 24, // horizontal left
            35, // vertical down
            19, 11, // vertical up
            36, 45, 54, 63, // diagonal to bottom right
            20, 13, // diagonal to top right
            34, 41, // diagonal to bottom left
            18, 9, // diagonal to top left
        ];
        assert_eq!(moves, expected);
    }
}
