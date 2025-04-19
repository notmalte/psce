use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr};

use crate::{color::Color, piece::Piece};

/*
    a   b   c   d   e   f   g   h
  ┌───┬───┬───┬───┬───┬───┬───┬───┐
8 │ 56│ 57│ 58│ 59│ 60│ 61│ 62│ 63│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
7 │ 48│ 49│ 50│ 51│ 52│ 53│ 54│ 55│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
6 │ 40│ 41│ 42│ 43│ 44│ 45│ 46│ 47│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
5 │ 32│ 33│ 34│ 35│ 36│ 37│ 38│ 39│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
4 │ 24│ 25│ 26│ 27│ 28│ 29│ 30│ 31│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
3 │ 16│ 17│ 18│ 19│ 20│ 21│ 22│ 23│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
2 │ 8 │ 9 │ 10│ 11│ 12│ 13│ 14│ 15│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
1 │ 0 │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │
  └───┴───┴───┴───┴───┴───┴───┴───┘
*/

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn get(&self, sq: u8) -> bool {
        self.0 & (1u64 << sq) != 0
    }

    pub fn set(&mut self, sq: u8) {
        self.0 |= 1u64 << sq;
    }

    pub fn clear(&mut self, sq: u8) {
        self.0 &= !(1u64 << sq);
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            let sq = self.0.trailing_zeros() as u8;
            self.clear(sq);
            Some(sq)
        }
    }

    pub const fn count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    pub const fn not(self) -> Self {
        Self(!self.0)
    }

    pub const fn shl(self, rhs: usize) -> Self {
        Self(self.0 << rhs)
    }

    pub const fn shr(self, rhs: usize) -> Self {
        Self(self.0 >> rhs)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.bitor(rhs)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.bitand(rhs)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

impl Shl<usize> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shl(rhs)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shr(rhs)
    }
}

pub const fn sq(rank: u8, file: u8) -> u8 {
    rank * 8 + file
}

pub fn sq_from_str(s: &str) -> Option<u8> {
    let chars: Vec<_> = s.to_lowercase().chars().collect();

    if chars.len() != 2 {
        return None;
    }

    let file = chars[0];
    let rank = chars[1];

    if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
        return None;
    }

    Some(sq(rank as u8 - b'1', file as u8 - b'a'))
}

pub const FILE_A: Bitboard = Bitboard(0x0101010101010101u64);
pub const FILE_B: Bitboard = FILE_A.shl(1);
pub const FILE_G: Bitboard = FILE_A.shl(6);
pub const FILE_H: Bitboard = FILE_A.shl(7);

pub const NOT_FILE_A: Bitboard = FILE_A.not();
pub const NOT_FILE_H: Bitboard = FILE_H.not();
pub const NOT_FILE_AB: Bitboard = (FILE_A.bitor(FILE_B)).not();
pub const NOT_FILE_GH: Bitboard = (FILE_G.bitor(FILE_H)).not();

pub const RANK_1: Bitboard = Bitboard(0x00000000000000FFu64);
pub const RANK_2: Bitboard = RANK_1.shl(8);
pub const RANK_7: Bitboard = RANK_1.shl(48);
pub const RANK_8: Bitboard = RANK_1.shl(56);

#[derive(Clone)]
pub struct BitboardContainer {
    pieces: [[Bitboard; 6]; 2],
    colors: [Bitboard; 2],
    all: Bitboard,
}

impl BitboardContainer {
    pub fn empty() -> Self {
        Self {
            pieces: [[Bitboard::empty(); 6]; 2],
            colors: [Bitboard::empty(); 2],
            all: Bitboard::empty(),
        }
    }

    pub fn piece(&self, color: Color, piece: Piece) -> Bitboard {
        self.pieces[color as usize][piece as usize]
    }

    pub fn piece_mut(&mut self, color: Color, piece: Piece) -> &mut Bitboard {
        &mut self.pieces[color as usize][piece as usize]
    }

    pub fn color(&self, color: Color) -> Bitboard {
        self.colors[color as usize]
    }

    pub fn color_mut(&mut self, color: Color) -> &mut Bitboard {
        &mut self.colors[color as usize]
    }

    pub fn all(&self) -> Bitboard {
        self.all
    }

    pub fn all_mut(&mut self) -> &mut Bitboard {
        &mut self.all
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let mut bb = Bitboard::empty();
        bb.set(sq(0, 0));
        bb.set(sq(0, 7));
        bb.set(sq(7, 0));
        bb.set(sq(7, 7));
        assert_eq!(bb.count(), 4);

        assert_eq!(FILE_A.count(), 8);

        assert_eq!(RANK_1.count(), 8);
    }
}
