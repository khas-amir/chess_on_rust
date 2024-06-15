use crate::{
    board::Board,
    piece::PieceType,
    utils::{coords_to_index, index_to_coords, is_valid_move},
};

pub fn get_knight_moves(index: usize, board: &Board) -> Vec<usize> {
    let piece = match &board.squares[index].piece {
        Some(piece) => piece,
        None => panic!("No piece at index"),
    };

    if piece.piece_type != PieceType::Knight {
        panic!("Piece is not a knight");
    }

    let mut moves = Vec::new();
    let (x, y) = index_to_coords(index);

    let possible_moves: [(i8, i8); 8] = [
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];

    for (dx, dy) in possible_moves {
        let new_x = x as i8 + dx;
        let new_y = y as i8 + dy;

        if is_valid_move(new_x, new_y, board, &piece.color) {
            let new_index = coords_to_index(new_x as usize, new_y as usize);
            moves.push(new_index);
        }
    }
    moves
}
