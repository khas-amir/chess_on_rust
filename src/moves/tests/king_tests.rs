#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::moves::king::get_king_moves;
    use crate::piece::get_black_piece;
    use crate::piece::get_white_piece;
    use crate::piece::PieceType;

    #[test]
    fn test_get_king_moves_initial() {
        let board = Board::new();
        let moves = get_king_moves(4, &board);
        let expected = vec![];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_king_moves_from_edge() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_black_piece(PieceType::King));
        let moves = get_king_moves(0, &board);
        let expected = vec![1, 8, 9];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_king_moves_from_corner() {
        let mut board = Board::empty();
        board.squares[63].piece = Some(get_black_piece(PieceType::King));
        let moves = get_king_moves(63, &board);
        let expected = vec![62, 55, 54];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_king_moves_from_center() {
        let mut board = Board::empty();
        board.squares[27].piece = Some(get_black_piece(PieceType::King));
        let moves = get_king_moves(27, &board);
        let expected = vec![28, 26, 35, 19, 36, 20, 34, 18];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_king_moves_blocks_by_ally() {
        let mut board = Board::empty();
        board.squares[2].piece = Some(get_black_piece(PieceType::King));
        board.squares[3].piece = Some(get_black_piece(PieceType::Rook));
        let moves = get_king_moves(2, &board);
        let expected = vec![1, 10, 11, 9];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_king_moves_blocks_by_enemy() {
        let mut board = Board::empty();
        board.squares[18].piece = Some(get_black_piece(PieceType::King));
        board.squares[10].piece = Some(get_white_piece(PieceType::Rook));
        let moves = get_king_moves(18, &board);
        let expected = vec![19, 17, 26, 10, 27, 11, 25, 9];
        assert_eq!(moves, expected);
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
