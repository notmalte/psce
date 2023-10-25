use crate::{color::Color, ply::PlyKind};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

impl CastlingRights {
    pub fn all() -> Self {
        return Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        };
    }

    pub fn none() -> Self {
        return Self {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        };
    }

    pub fn get(&self, color: Color, side: CastlingSide) -> bool {
        match (color, side) {
            (Color::White, CastlingSide::Kingside) => self.white_kingside,
            (Color::White, CastlingSide::Queenside) => self.white_queenside,
            (Color::Black, CastlingSide::Kingside) => self.black_kingside,
            (Color::Black, CastlingSide::Queenside) => self.black_queenside,
        }
    }

    pub fn set(&mut self, color: Color, side: CastlingSide, value: bool) {
        match (color, side) {
            (Color::White, CastlingSide::Kingside) => self.white_kingside = value,
            (Color::White, CastlingSide::Queenside) => self.white_queenside = value,
            (Color::Black, CastlingSide::Kingside) => self.black_kingside = value,
            (Color::Black, CastlingSide::Queenside) => self.black_queenside = value,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}

impl CastlingSide {
    pub fn to_string(self) -> String {
        match self {
            Self::Kingside => "Kingside",
            Self::Queenside => "Queenside",
        }
        .to_string()
    }

    pub fn to_ply_kind(self) -> PlyKind {
        match self {
            Self::Kingside => PlyKind::CastleKingside,
            Self::Queenside => PlyKind::CastleQueenside,
        }
    }
}
