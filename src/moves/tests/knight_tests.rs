#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::piece::get_white_piece;
    use crate::piece::PieceType;
    use crate::{moves::knight::get_knight_moves, piece::get_black_piece};

    #[test]
    fn test_knight_initial_moves() {
        let board = Board::new();
        let moves = get_knight_moves(1, &board);
        assert!(moves.contains(&16));
        assert!(moves.contains(&18));
    }

    #[test]
    fn test_knight_moves_capture() {
        let mut board = Board::new();
        board.squares[0].piece = Some(get_white_piece(PieceType::Knight));
        board.squares[10].piece = Some(get_black_piece(PieceType::Pawn));
        board.squares[17].piece = Some(get_black_piece(PieceType::Pawn));
        let moves = get_knight_moves(0, &board);
        let expected_moves = vec![10, 17];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_knight_moves_blocked_by_own_piece() {
        let mut board = Board::new();
        board.squares[0].piece = Some(get_white_piece(PieceType::Knight));
        board.squares[10].piece = Some(get_white_piece(PieceType::Pawn));
        board.squares[17].piece = Some(get_white_piece(PieceType::Pawn));
        let moves = get_knight_moves(0, &board);
        let expected_moves: Vec<usize> = vec![];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_knight_moves_on_edge_of_board() {
        let mut board = Board::new();
        let index = 8 * 7;
        board.squares[index].piece = Some(get_white_piece(PieceType::Knight));
        board.squares[41].piece = None;
        board.squares[50].piece = None;
        let moves = get_knight_moves(index, &board);
        let expected_moves = vec![41, 50];
        assert_eq!(moves, expected_moves);
    }
}
