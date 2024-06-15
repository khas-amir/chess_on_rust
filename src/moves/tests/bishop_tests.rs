#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        moves::bishop::get_bishop_moves,
        piece::{get_black_piece, get_white_piece, PieceType},
    };

    #[test]
    fn test_get_bishop_moves_initial() {
        let board = Board::new();
        let moves = get_bishop_moves(2, &board);
        let expected = vec![];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_bishop_moves_from_edge() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_black_piece(PieceType::Bishop));
        let moves = get_bishop_moves(0, &board);
        let expected = vec![9, 18, 27, 36, 45, 54, 63];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_bishop_moves_from_bottom_right_edge() {
        let mut board = Board::empty();
        board.squares[63].piece = Some(get_black_piece(PieceType::Bishop));
        let moves = get_bishop_moves(63, &board);
        let expected = vec![54, 45, 36, 27, 18, 9, 0];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_bishop_moves_when_blocks_by_ally() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_black_piece(PieceType::Bishop));
        board.squares[36].piece = Some(get_black_piece(PieceType::Pawn));
        let moves = get_bishop_moves(0, &board);
        let expected = vec![9, 18, 27];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_bishop_moves_when_can_capture() {
        let mut board = Board::empty();
        board.squares[0].piece = Some(get_black_piece(PieceType::Bishop));
        board.squares[36].piece = Some(get_white_piece(PieceType::Pawn));
        let moves = get_bishop_moves(0, &board);
        let expected = vec![9, 18, 27, 36];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_bishop_in_the_middle() {
        let mut board = Board::empty();
        board.squares[36].piece = Some(get_black_piece(PieceType::Bishop));
        let moves = get_bishop_moves(36, &board);
        let expected = vec![
            45, 54, 63, // diagonal to right bottom
            29, 22, 15, // diagonal to right top
            43, 50, 57, //diagonal to left bottom
            27, 18, 9, 0, //diagonal to left top
        ];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_bishop_moves_can_capture_block() {
        let mut board = Board::empty();
        board.squares[36].piece = Some(get_black_piece(PieceType::Bishop));
        board.squares[27].piece = Some(get_black_piece(PieceType::Pawn));
        board.squares[29].piece = Some(get_white_piece(PieceType::Pawn));
        let moves = get_bishop_moves(36, &board);
        let expected = vec![
            45, 54, 63, // diagonal to right bottom
            29, //diagonal to left bottom
            43, 50, 57,
        ];
        assert_eq!(moves, expected);
    }
}
