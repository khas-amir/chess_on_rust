use crate::board::Board;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
        }
    }

    pub fn input_from_user() -> ((usize, usize), (usize, usize)) {
        println!("What piece do you want to move? (x,y) (where x is the row, and y is the column)");
        let mut from_raw = String::new();
        std::io::stdin()
            .read_line(&mut from_raw)
            .expect("Failed to read line");

        println!("Where do you want to move? (x,y) (where x is the row, and y is the column)");
        let mut to_raw = String::new();
        std::io::stdin()
            .read_line(&mut to_raw)
            .expect("Failed to read line");

        let v_from: Vec<&str> = from_raw.trim().split(',').collect();
        let from = (
            v_from[0].parse::<usize>().unwrap(),
            v_from[1].parse::<usize>().unwrap(),
        );

        let v_to: Vec<&str> = to_raw.trim().split(',').collect();
        let to = (
            v_to[0].parse::<usize>().unwrap(),
            v_to[1].parse::<usize>().unwrap(),
        );
        (from, to)
    }

    pub fn start(&mut self) {
        println!("\n{}", self.board);
        let (from, to) = Self::input_from_user();
        self.board.move_piece(from, to);
        println!("\n{}", self.board);
    }
}
