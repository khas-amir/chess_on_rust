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

    pub fn input_str(msg: &str) -> String {
        println!("{}", msg);
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .expect("Failed to read line");
        s
    }

    pub fn input_from_to_coord_str() -> ((usize, usize), (usize, usize)) {
        let from_raw = Self::input_str(
            "What piece do you want to move? (x,y) (where x is the row, and y is the column)",
        );
        let to_raw = Self::input_str(
            "Where do you want to move? (x,y) (where x is the row, and y is the column)",
        );
        let from = Self::parse_str(&from_raw);
        let to = Self::parse_str(&to_raw);
        (from, to)
    }

    pub fn start(&mut self) {
        println!("\n{}", self.board);
        let (from, to) = Self::input_from_to_coord_str();
        self.board.move_piece(from, to);
        println!("\n{}", self.board);
    }
}
