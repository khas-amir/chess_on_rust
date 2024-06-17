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

pub fn get_diagonal_moves(x: usize, y: usize, to_right: bool, to_down: bool) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let range = get_range(x, to_right);
    let range2 = get_range(y, to_down);

    for (dx, dy) in range.zip(range2) {
        moves.push((dx as i8, dy as i8));
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coords_to_index() {
        assert_eq!(coords_to_index(0, 0), 0);
        assert_eq!(coords_to_index(1, 1), 9);
        assert_eq!(coords_to_index(7, 7), 63);
    }

    #[test]
    fn test_index_to_coords() {
        assert_eq!(index_to_coords(10), (2, 1));
        assert_eq!(index_to_coords(63), (7, 7));
        assert_eq!(index_to_coords(0), (0, 0));
        assert_eq!(index_to_coords(9), (1, 1));
        assert_eq!(index_to_coords(5), (5, 0));
    }

    #[test]
    fn test_get_horizontal_moves_in_top_left_edge() {
        let moves = get_horizontal_moves(0, 0, true);
        let expected = vec![(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_horizontal_moves_in_top_right_edge() {
        let moves = get_horizontal_moves(7, 0, false);
        let expected = vec![(6, 0), (5, 0), (4, 0), (3, 0), (2, 0), (1, 0), (0, 0)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_horizontal_moves_in_the_middle() {
        let moves = get_horizontal_moves(3, 3, true);
        let expected = vec![(4, 3), (5, 3), (6, 3), (7, 3)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_vertical_moves_in_top_left_edge() {
        let moves = get_vertical_moves(0, 0, true);
        let expected = vec![(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_vertical_moves_in_top_right_edge() {
        let moves = get_vertical_moves(7, 0, true);
        let expected = vec![(7, 1), (7, 2), (7, 3), (7, 4), (7, 5), (7, 6), (7, 7)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_vertical_moves_in_the_middle() {
        let moves = get_vertical_moves(3, 3, false);
        let expected = vec![(3, 2), (3, 1), (3, 0)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_get_diagonal_moves() {
        let moves = get_diagonal_moves(0, 0, true, true);
        let expected = vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)];
        assert_eq!(moves, expected);
    }
}
