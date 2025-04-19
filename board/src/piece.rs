use strum::EnumIter;

use crate::color::Color;

#[derive(Copy, Clone, PartialEq, Debug, EnumIter)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'p' => Some(Self::Pawn),
            'n' => Some(Self::Knight),
            'b' => Some(Self::Bishop),
            'r' => Some(Self::Rook),
            'q' => Some(Self::Queen),
            'k' => Some(Self::King),
            _ => None,
        }
    }

    pub fn to_char(self, color: Color) -> char {
        match (self, color) {
            (Self::Pawn, Color::White) => 'P',
            (Self::Pawn, Color::Black) => 'p',
            (Self::Knight, Color::White) => 'N',
            (Self::Knight, Color::Black) => 'n',
            (Self::Bishop, Color::White) => 'B',
            (Self::Bishop, Color::Black) => 'b',
            (Self::Rook, Color::White) => 'R',
            (Self::Rook, Color::Black) => 'r',
            (Self::Queen, Color::White) => 'Q',
            (Self::Queen, Color::Black) => 'q',
            (Self::King, Color::White) => 'K',
            (Self::King, Color::Black) => 'k',
        }
    }
}
