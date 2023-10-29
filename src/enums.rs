use std::fmt::Display;
use strum::{Display, EnumIter, FromRepr};

#[derive(Clone, Copy, PartialEq, Display)]
pub enum Color {
    White,
    Black,
    Both,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RookOrBishop {
    Rook,
    Bishop,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CastlingRights {
    WhiteKingside = 0b0001,
    WhiteQueenside = 0b0010,
    BlackKingside = 0b0100,
    BlackQueenside = 0b1000,
    None = 0b000,
    All = 0b1111,
}

impl CastlingRights {
    pub fn flag(&self) -> u8 {
        (*self) as u8
    }

    pub fn flags_to_string(flags: u8) -> String {
        if flags == Self::None.flag() {
            return "-".to_string();
        }

        let mut result = String::new();

        if flags & Self::WhiteKingside.flag() != 0 {
            result.push('K');
        }

        if flags & Self::WhiteQueenside.flag() != 0 {
            result.push('Q');
        }

        if flags & Self::BlackKingside.flag() != 0 {
            result.push('k');
        }

        if flags & Self::BlackQueenside.flag() != 0 {
            result.push('q');
        }

        result
    }

    pub fn string_to_flags(string: String) -> Result<u8, String> {
        if string == "-" {
            return Ok(Self::None.flag());
        }

        let mut result = 0;

        for c in string.chars() {
            result |= match c {
                'K' => Self::WhiteKingside.flag(),
                'Q' => Self::WhiteQueenside.flag(),
                'k' => Self::BlackKingside.flag(),
                'q' => Self::BlackQueenside.flag(),
                _ => return Err(format!("Invalid castling rights: {}", string)),
            }
        }

        Ok(result)
    }
}

#[derive(Clone, Copy, PartialEq, EnumIter, FromRepr)]
pub enum PieceAndColor {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl PieceAndColor {
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'P' => Ok(Self::WhitePawn),
            'N' => Ok(Self::WhiteKnight),
            'B' => Ok(Self::WhiteBishop),
            'R' => Ok(Self::WhiteRook),
            'Q' => Ok(Self::WhiteQueen),
            'K' => Ok(Self::WhiteKing),
            'p' => Ok(Self::BlackPawn),
            'n' => Ok(Self::BlackKnight),
            'b' => Ok(Self::BlackBishop),
            'r' => Ok(Self::BlackRook),
            'q' => Ok(Self::BlackQueen),
            'k' => Ok(Self::BlackKing),
            _ => Err(format!("Invalid piece: {}", c)),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::WhitePawn => 'P',
            Self::WhiteKnight => 'N',
            Self::WhiteBishop => 'B',
            Self::WhiteRook => 'R',
            Self::WhiteQueen => 'Q',
            Self::WhiteKing => 'K',
            Self::BlackPawn => 'p',
            Self::BlackKnight => 'n',
            Self::BlackBishop => 'b',
            Self::BlackRook => 'r',
            Self::BlackQueen => 'q',
            Self::BlackKing => 'k',
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::WhitePawn
            | Self::WhiteKnight
            | Self::WhiteBishop
            | Self::WhiteRook
            | Self::WhiteQueen
            | Self::WhiteKing => Color::White,
            Self::BlackPawn
            | Self::BlackKnight
            | Self::BlackBishop
            | Self::BlackRook
            | Self::BlackQueen
            | Self::BlackKing => Color::Black,
        }
    }
}

impl Display for PieceAndColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceAndColor::WhitePawn => "♙",
                PieceAndColor::WhiteKnight => "♘",
                PieceAndColor::WhiteBishop => "♗",
                PieceAndColor::WhiteRook => "♖",
                PieceAndColor::WhiteQueen => "♕",
                PieceAndColor::WhiteKing => "♔",
                PieceAndColor::BlackPawn => "♟︎",
                PieceAndColor::BlackKnight => "♞",
                PieceAndColor::BlackBishop => "♝",
                PieceAndColor::BlackRook => "♜",
                PieceAndColor::BlackQueen => "♛",
                PieceAndColor::BlackKing => "♚",
            }
        )
    }
}
