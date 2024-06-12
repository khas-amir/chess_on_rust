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

    pub fn parse_str(str: &str) -> (usize, usize) {
        let v: Vec<&str> = str.trim().split(',').collect();
        let first = match v[0].parse::<usize>() {
            Ok(x) => x,
            Err(_) => panic!("Invalid first number"),
        };

        let second = match v[1].parse::<usize>() {
            Ok(x) => x,
            Err(_) => panic!("Invalid second number"),
        };

        (first, second)
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

        let from = Self::parse_str(&from_raw);
        let to = Self::parse_str(&to_raw);
        (from, to)
    }

    pub fn start(&mut self) {
        println!("\n{}", self.board);
        let (from, to) = Self::input_from_user();
        self.board.move_piece(from, to);
        println!("\n{}", self.board);
    }
}
