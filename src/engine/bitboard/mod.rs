use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr},
};

mod container;
mod square;

pub use container::*;
pub use square::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Bitboard(u64);

/*
    a   b   c   d   e   f   g   h
  ┌───┬───┬───┬───┬───┬───┬───┬───┐
8 │ 0 │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │
  ├───┼───┼───┼───┼───┼───┼───┼───┤
7 │ 8 │ 9 │ 10│ 11│ 12│ 13│ 14│ 15│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
6 │ 16│ 17│ 18│ 19│ 20│ 21│ 22│ 23│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
5 │ 24│ 25│ 26│ 27│ 28│ 29│ 30│ 31│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
4 │ 32│ 33│ 34│ 35│ 36│ 37│ 38│ 39│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
3 │ 40│ 41│ 42│ 43│ 44│ 45│ 46│ 47│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
2 │ 48│ 49│ 50│ 51│ 52│ 53│ 54│ 55│
  ├───┼───┼───┼───┼───┼───┼───┼───┤
1 │ 56│ 57│ 58│ 59│ 60│ 61│ 62│ 63│
  └───┴───┴───┴───┴───┴───┴───┴───┘
*/

impl Bitboard {
    pub fn to_repr(self) -> u64 {
        self.0
    }

    pub fn empty() -> Self {
        Bitboard(0)
    }

    pub fn all_squares() -> BitboardAllSquaresIterator {
        BitboardAllSquaresIterator::new()
    }

    pub fn get(self, square: Square) -> bool {
        !(self & square.to_bb()).is_empty()
    }

    pub fn set(&mut self, square: Square) {
        *self |= square.to_bb();
    }

    pub fn clear(&mut self, square: Square) {
        *self &= !square.to_bb();
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn is_not_empty(self) -> bool {
        !self.is_empty()
    }

    pub fn last_square(self) -> Option<Square> {
        Square::from_repr(self.0.trailing_zeros() as u8)
    }

    pub fn pop_square(&mut self) -> Option<Square> {
        let square = self.last_square()?;

        self.clear(square);

        Some(square)
    }

    pub fn squares(self) -> BitboardSetSquaresIterator {
        BitboardSetSquaresIterator::new(self)
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

    pub const fn count_ones(self) -> u8 {
        self.0.count_ones() as u8
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  a b c d e f g h")?;

        for y in 0..8 {
            write!(f, "{} ", 8 - y)?;

            for x in 0..8 {
                if x != 0 {
                    write!(f, " ")?;
                }

                if self.get(Square::from_xy(x, y).unwrap()) {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }

            writeln!(f)?;
        }

        write!(f, "\ndec: {}", self.0)?;

        Ok(())
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

pub struct BitboardAllSquaresIterator {
    index: u8,
}

impl BitboardAllSquaresIterator {
    fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for BitboardAllSquaresIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 64 {
            None
        } else {
            let result = Some(Square::from_repr_unchecked(self.index));
            self.index += 1;
            result
        }
    }
}

pub struct BitboardSetSquaresIterator {
    bitboard: Bitboard,
}

impl BitboardSetSquaresIterator {
    fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }
}

impl Iterator for BitboardSetSquaresIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.bitboard.pop_square()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_get_set_clear() {
        let mut bb = Bitboard::empty();

        assert!(!bb.get(Square::from_repr(0).unwrap()));
        assert!(!bb.get(Square::from_repr(1).unwrap()));
        assert!(!bb.get(Square::from_repr(63).unwrap()));

        bb.set(Square::from_repr(0).unwrap());

        assert!(bb.get(Square::from_repr(0).unwrap()));
        assert!(!bb.get(Square::from_repr(1).unwrap()));
        assert!(!bb.get(Square::from_repr(63).unwrap()));

        bb.set(Square::from_repr(63).unwrap());

        assert!(bb.get(Square::from_repr(0).unwrap()));
        assert!(!bb.get(Square::from_repr(1).unwrap()));
        assert!(bb.get(Square::from_repr(63).unwrap()));

        bb.clear(Square::from_repr(0).unwrap());

        assert!(!bb.get(Square::from_repr(0).unwrap()));
        assert!(!bb.get(Square::from_repr(1).unwrap()));
        assert!(bb.get(Square::from_repr(63).unwrap()));

        bb.set(Square::from_repr(0).unwrap());
        bb.clear(Square::from_repr(63).unwrap());
        bb.clear(Square::from_repr(63).unwrap());

        assert!(bb.get(Square::from_repr(0).unwrap()));
        assert!(!bb.get(Square::from_repr(1).unwrap()));
        assert!(!bb.get(Square::from_repr(63).unwrap()));
    }
}
