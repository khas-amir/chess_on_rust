use crate::{board::Board, piece::Color};

pub fn index_to_coords(index: usize) -> (usize, usize) {
    (index % 8, index / 8)
}

pub fn coords_to_index(x: usize, y: usize) -> usize {
    y * 8 + x
}

pub fn is_valid_move(new_x: i8, new_y: i8, board: &Board, color: &Color) -> bool {
    if !(0..8).contains(&new_x) || !(0..8).contains(&new_y) {
        return false;
    }

    let possible_square = &board.squares[coords_to_index(new_x as usize, new_y as usize)];
    match &possible_square.piece {
        Some(piece) => &piece.color != color,
        None => true,
    }
}

pub fn get_range(to: usize, is_rev: bool) -> Box<dyn Iterator<Item = usize>> {
    if is_rev {
        Box::new(to + 1..=7)
    } else {
        Box::new((0..to).rev())
    }
}

pub fn get_horizontal_moves(x: usize, y: usize, to_right: bool) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let range = get_range(x, to_right);

    for dx in range {
        moves.push((dx as i8, y as i8));
    }
    moves
}

pub fn get_vertical_moves(x: usize, y: usize, to_down: bool) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let range = get_range(y, to_down);
    for dy in range {
        moves.push((x as i8, dy as i8));
    }
    moves
}
