use crate::{color::Color, piece::Piece};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Square {
    Empty,
    Occupied(Color, Piece),
}

impl Square {
    pub fn to_string(self) -> String {
        match self {
            Self::Empty => " ".to_string(),
            Self::Occupied(c, p) => p.to_character(c),
        }
    }
}
