use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChessBoard {
    board: [[String; 8]; 8],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let initial_board: [[String; 8]; 8] = [
            ["R`".to_owned(), "N`".to_owned(), "B`".to_owned(), "Q`".to_owned(), "K`".to_owned(), "B`".to_owned(), "N`".to_owned(), "R`".to_owned()],
            ["P`".to_owned(), "P`".to_owned(), "P`".to_owned(), "P`".to_owned(), "P`".to_owned(), "P`".to_owned(), "P`".to_owned(), "P`".to_owned()],
            [".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned()],
            [".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned()],
            [".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned()],
            [".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned(), ".".to_owned()],
            ["P".to_owned(), "P".to_owned(), "P".to_owned(), "P".to_owned(), "P".to_owned(), "P".to_owned(), "P".to_owned(), "P".to_owned()],
            ["R".to_owned(), "N".to_owned(), "B".to_owned(), "Q".to_owned(), "K".to_owned(), "B".to_owned(), "N".to_owned(), "R".to_owned()]
        ];

        ChessBoard {
            board: initial_board,
        }
    }

    pub fn new_from_board(board: Option<[[String; 8]; 8]>) -> ChessBoard {
        match board {
            Some(b) => {
                ChessBoard {
                    board: b,
                }
            },
            None => {
                ChessBoard::new()
            }
        }
    }

    pub fn make_move(&self, from: usize, to: &str) -> ChessBoard {
        let mut new_board: ChessBoard = self.clone();
        let from_row = from / 8;
        let from_col = from % 8;
        let to_row = to.chars().nth(0).unwrap() as usize - 65;
        let to_col = to.chars().nth(1).unwrap() as usize - 49;

        new_board.board[to_row][to_col] = new_board.board[from_row][from_col].clone();
        new_board.board[from_row][from_col] = ".".to_owned();

        return new_board;
    }
}

impl ChessBoard {
    fn is_draw(&self) -> bool {
        return false;
    }

    fn is_checkmate(&self) -> bool {
        return false;
    }
    
    pub fn board_score(&self) -> u32 {
        if ChessBoard::is_draw(&self) {
            return 0;
        }

        if ChessBoard::is_checkmate(&self) {
            return 2000;
        }

        let mut score = 0;
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell.as_str() {
                    "P" => score += 1,
                    "P`" => score -= 1,
                    "R" => score += 5,
                    "R`" => score -= 5,
                    "N" => score += 3,
                    "N`" => score -= 3,
                    "B" => score += 3,
                    "B`" => score -= 3,
                    "Q" => score += 9,
                    "Q`" => score -= 9,
                    "K" => score += 1000,
                    "K`" => score -= 1000,
                    _ => (),
                }
            }
        }
        return score;
    }

}

impl ChessBoard {
   pub fn minMax(&self, depth: u8, is_maximizing: bool) -> i32 {
        let mut best_move = None;
        let mut best_score = -10000;

        for row in self.board.iter() {
            for (index, cell) in row.iter().enumerate() {
                let mut new_board = self.clone();
                let mut new_board = new_board.make_move(index, "A1");
                let score = ChessBoard::minMax(&new_board, depth - 1, !is_maximizing);
                if score > best_score {
                    best_score = score;
                    best_move = Some(index);
                }
            }
        }

        return best_score;
    }
}