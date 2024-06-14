#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::moves::pawn::get_pawn_moves;
    use crate::piece::get_black_piece;
    use crate::piece::get_white_piece;
    use crate::piece::PieceType::Pawn;

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
