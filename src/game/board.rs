use serde::{Deserialize, Serialize};

use crate::game::piece::Piece;
use crate::game::piece::Piece::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChessBoard {
    board: [[Piece; 8]; 8],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let initial_board: [[Piece; 8]; 8] = [
            [
                Rook(false),
                Knight(false),
                Bishop(false),
                Queen(false),
                King(false),
                Bishop(false),
                Knight(false),
                Rook(false),
            ],
            [
                Pawn(false),
                Pawn(false),
                Pawn(false),
                Pawn(false),
                Pawn(false),
                Pawn(false),
                Pawn(false),
                Pawn(false),
            ],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [
                Pawn(true),
                Pawn(true),
                Pawn(true),
                Pawn(true),
                Pawn(true),
                Pawn(true),
                Pawn(true),
                Pawn(true),
            ],
            [
                Rook(true),
                Knight(true),
                Bishop(true),
                Queen(true),
                King(true),
                Bishop(true),
                Knight(true),
                Rook(true),
            ],
        ];

        ChessBoard {
            board: initial_board,
        }
    }

    pub fn copy(other: &ChessBoard) -> ChessBoard {
        ChessBoard {
            board: other.board.clone(),
        }
    }
}


impl ChessBoard {
    pub fn is_valid(&self) -> bool {
        for row in self.board.iter() {
            let mut white_king_count = 0;
            let mut white_queen_count = 0;
            let mut white_rook_count = 0;
            let mut white_bishop_count = 0;
            let mut white_knight_count = 0;
            let mut white_pawn_count = 0;


            let mut black_king_count = 0;
            let mut black_queen_count = 0;
            let mut black_rook_count = 0;
            let mut black_bishop_count = 0;
            let mut black_knight_count = 0;
            let mut black_pawn_count = 0;

            for piece in row.iter() {
                match piece {
                    Rook(c) => {
                        if *c {
                            white_rook_count += 1;
                        } else {
                            black_rook_count += 1;
                        }
                    },
                    Knight(c) => {
                        if *c {
                            white_knight_count += 1;
                        } else {
                            black_knight_count += 1;
                        }
                    },
                    Bishop(c) => {
                        if *c {
                            white_bishop_count += 1;
                        } else {
                            black_bishop_count += 1;
                        }
                    },
                    Queen(c) => {
                        if *c {
                            white_queen_count += 1;
                        } else {
                            black_queen_count += 1;
                        }
                    },
                    King(c) => {
                        if *c {
                            white_king_count += 1;
                        } else {
                            black_king_count += 1;
                        }
                    },
                    Pawn(c) => {
                        if *c {
                            white_pawn_count += 1;
                        } else {
                            black_pawn_count += 1;
                        }
                    },
                    Empty => (),
                }
            }

            if white_king_count > 1 || black_king_count > 1 {
                return false;
            }

            if white_queen_count > 1 || black_queen_count > 1 {
                return false;
            }

            if white_rook_count > 2 || black_rook_count > 2 {
                return false;
            }

            if white_bishop_count > 2 || black_bishop_count > 2 {
                return false;
            }

            if white_knight_count > 2 || black_knight_count > 2 {
                return false;
            }

            if white_pawn_count > 8 || black_pawn_count > 8 {
                return false;
            }
        }


        true
    }
}