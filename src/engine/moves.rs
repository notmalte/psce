use std::ops::BitOr;

use crate::engine::piece::Piece;

#[derive(Clone)]
pub struct Move {
    from: u8,
    to: u8,
    piece: Piece,
    promotion: Option<Piece>,
    flags: MoveFlags,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum MoveFlag {
    None = 0b0000,
    Capture = 0b0001,
    EnPassant = 0b0010,
    Castling = 0b0100,
    DoublePush = 0b1000,
}

impl MoveFlag {
    pub fn to_repr(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct MoveFlags(u8);

impl MoveFlags {
    pub fn none() -> Self {
        MoveFlags(MoveFlag::None.to_repr())
    }

    pub fn from_flag(flag: MoveFlag) -> Self {
        MoveFlags(flag.to_repr())
    }

    pub fn has(&self, flag: MoveFlag) -> bool {
        (self.0 & flag.to_repr()) != 0
    }
}

impl BitOr<MoveFlag> for MoveFlag {
    type Output = MoveFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        MoveFlags(self.to_repr() | rhs.to_repr())
    }
}

impl BitOr<MoveFlag> for MoveFlags {
    type Output = MoveFlags;

    fn bitor(self, rhs: MoveFlag) -> Self::Output {
        MoveFlags(self.0 | rhs.to_repr())
    }
}

impl BitOr<MoveFlags> for MoveFlag {
    type Output = MoveFlags;

    fn bitor(self, rhs: MoveFlags) -> Self::Output {
        MoveFlags(self.to_repr() | rhs.0)
    }
}

impl BitOr<MoveFlags> for MoveFlags {
    type Output = MoveFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        MoveFlags(self.0 | rhs.0)
    }
}
