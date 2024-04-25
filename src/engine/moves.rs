use std::{fmt::Display, ops::BitOr};

use crate::engine::{bitboard::Square, piece::Piece};

#[derive(Clone, Debug)]
pub struct Move {
    from: Square,
    to: Square,
    piece: Piece,
    promotion: Option<Piece>,
    flags: MoveFlags,
}

impl Move {
    pub fn new(
        from: Square,
        to: Square,
        piece: Piece,
        promotion: Option<Piece>,
        flags: MoveFlags,
    ) -> Self {
        Self {
            from,
            to,
            piece,
            promotion,
            flags,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.piece,
            self.from,
            if self.flags.is_capture() { "x" } else { "" },
            self.to,
            if let Some(promotion) = self.promotion {
                promotion.to_string()
            } else {
                "".to_string()
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MoveFlags(u8);

impl MoveFlags {
    pub const NONE: Self = MoveFlags(0b0000);
    pub const CAPTURE: Self = MoveFlags(0b0001);
    pub const EN_PASSANT: Self = MoveFlags(0b0010);
    pub const CASTLING: Self = MoveFlags(0b0100);
    pub const DOUBLE_PUSH: Self = MoveFlags(0b1000);

    pub fn has(&self, flags: MoveFlags) -> bool {
        (self.0 & flags.0) != 0
    }

    pub fn is_capture(&self) -> bool {
        self.has(MoveFlags::CAPTURE)
    }

    pub fn is_en_passant(&self) -> bool {
        self.has(MoveFlags::EN_PASSANT)
    }

    pub fn is_castling(&self) -> bool {
        self.has(MoveFlags::CASTLING)
    }

    pub fn is_double_push(&self) -> bool {
        self.has(MoveFlags::DOUBLE_PUSH)
    }
}

impl BitOr<MoveFlags> for MoveFlags {
    type Output = MoveFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        MoveFlags(self.0 | rhs.0)
    }
}
