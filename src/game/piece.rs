use serde::{Deserialize, Deserializer, Serialize, Serializer, de };
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Rook(bool),
    Knight(bool),
    Bishop(bool),
    Queen(bool),
    King(bool),
    Pawn(bool),
    Empty,
}

impl Serialize for Piece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Piece::Rook(true) => "R",
            Piece::Knight(true) => "N",
            Piece::Bishop(true) => "B",
            Piece::Queen(true) => "Q",
            Piece::King(true) => "K",
            Piece::Pawn(true) => "P",
            Piece::Rook(false) => "R`",
            Piece::Knight(false) => "N`",
            Piece::Bishop(false) => "B`",
            Piece::Queen(false) => "Q`",
            Piece::King(false) => "K`",
            Piece::Pawn(false) => "P`",
            Piece::Empty => ".",
        };

        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for Piece {
    fn deserialize<D>(deserializer: D) -> Result<Piece, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PieceVisitor;

        impl<'de> de::Visitor<'de> for PieceVisitor {
            type Value = Piece;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a chess piece represented as a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Piece, E>
            where
                E: de::Error,
            {
                match value {
                    "R" => Ok(Piece::Rook(true)),
                    "N" => Ok(Piece::Knight(true)),
                    "B" => Ok(Piece::Bishop(true)),
                    "Q" => Ok(Piece::Queen(true)),
                    "K" => Ok(Piece::King(true)),
                    "P" => Ok(Piece::Pawn(true)),
                    "R`" => Ok(Piece::Rook(false)),
                    "N`" => Ok(Piece::Knight(false)),
                    "B`" => Ok(Piece::Bishop(false)),
                    "Q`" => Ok(Piece::Queen(false)),
                    "K`" => Ok(Piece::King(false)),
                    "P`" => Ok(Piece::Pawn(false)),
                    "." => Ok(Piece::Empty),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &[
                            "R", "N", "B", "Q", "K", "P", "R`", "N`", "B`", "Q`", "K`", "P`", ".",
                        ],
                    )),
                }
            }
        }

        deserializer.deserialize_str(PieceVisitor)
    }
}
