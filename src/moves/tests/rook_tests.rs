#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::moves::rook::get_rook_moves;
    use crate::piece::{get_black_piece, get_white_piece, PieceType};

    #[test]
    fn test_rook_initial_move() {
        let board = Board::new();
        let moves = get_rook_moves(0, &board);
        let expected_moves = vec![];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_rook_moves_only_horizontal_right() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_white_piece(PieceType::Rook));
        board.squares[8].piece = Some(get_white_piece(PieceType::Rook));
        let moves = get_rook_moves(0, &board);
        let expected_moves = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_rook_moves_only_horizontal_left() {
        let mut board = Board::empty();
        board.squares[7].piece = Some(get_white_piece(PieceType::Rook));
        board.squares[15].piece = Some(get_white_piece(PieceType::Rook));
        let moves = get_rook_moves(7, &board);
        let expected_moves = vec![6, 5, 4, 3, 2, 1, 0];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_rook_moves_only_vertical_down() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_white_piece(PieceType::Rook));
        board.squares[1].piece = Some(get_white_piece(PieceType::Rook));
        let moves = get_rook_moves(0, &board);
        let expected_moves = vec![8, 16, 24, 32, 40, 48, 56];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_rook_moves_only_vertical_up() {
        let mut board = Board::empty();
        board.squares[56].piece = Some(get_white_piece(PieceType::Rook));
        board.squares[57].piece = Some(get_white_piece(PieceType::Rook));
        let moves = get_rook_moves(56, &board);
        let expected_moves = vec![48, 40, 32, 24, 16, 8, 0];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_rook_moves_blocked_by_own_piece() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_white_piece(PieceType::Rook));
        board.squares[3].piece = Some(get_white_piece(PieceType::Pawn));
        board.squares[24].piece = Some(get_white_piece(PieceType::Pawn));
        let moves = get_rook_moves(0, &board);
        let expected_moves = vec![
            1, 2, // Horizontal moves until blocked
            8, 16, // Vertical moves until blocked
        ];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_rook_moves_capture_enemy() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_white_piece(PieceType::Rook));
        board.squares[3].piece = Some(get_black_piece(PieceType::Pawn));
        board.squares[24].piece = Some(get_black_piece(PieceType::Pawn));
        let moves = get_rook_moves(0, &board);
        let expected_moves = vec![
            1, 2, 3, // Horizontal moves until capture
            8, 16, 24, // Vertical moves until capture
        ];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_rook_moves_on_edge_of_board() {
        let mut board = Board::empty();
        board.squares[7].piece = Some(get_white_piece(PieceType::Rook));
        let moves = get_rook_moves(7, &board);
        let expected_moves = vec![
            6, 5, 4, 3, 2, 1, 0, // Horizontal moves
            15, 23, 31, 39, 47, 55, 63, // Vertical moves
        ];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_rook_moves_in_middle_of_board() {
        let mut board = Board::empty();
        board.squares[27].piece = Some(get_white_piece(PieceType::Rook));
        let moves = get_rook_moves(27, &board);
        let expected_moves = vec![
            28, 29, 30, 31, // horizontal right
            26, 25, 24, // horizontal left
            35, 43, 51, 59, // vertical down
            19, 11, 3, // vertical up
        ];
        assert_eq!(moves, expected_moves);
    }
}
