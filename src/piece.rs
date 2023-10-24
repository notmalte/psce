use crate::color::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    pub fn to_character(self, color: Color) -> String {
        match (color, self) {
            (Color::White, Self::Pawn) => "♙",
            (Color::White, Self::Rook) => "♖",
            (Color::White, Self::Knight) => "♘",
            (Color::White, Self::Bishop) => "♗",
            (Color::White, Self::Queen) => "♕",
            (Color::White, Self::King) => "♔",
            (Color::Black, Self::Pawn) => "♟︎",
            (Color::Black, Self::Rook) => "♜",
            (Color::Black, Self::Knight) => "♞",
            (Color::Black, Self::Bishop) => "♝",
            (Color::Black, Self::Queen) => "♛",
            (Color::Black, Self::King) => "♚",
        }
        .to_string()
    }

    pub fn to_name(self) -> String {
        match self {
            Self::Pawn => "Pawn",
            Self::Rook => "Rook",
            Self::Knight => "Knight",
            Self::Bishop => "Bishop",
            Self::Queen => "Queen",
            Self::King => "King",
        }
        .to_string()
    }
}
