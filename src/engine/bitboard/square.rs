use std::{
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
};

use crate::engine::bitboard::Bitboard;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Square(u8);

#[allow(dead_code)]
impl Square {
    pub const A8: Square = Square(0);
    pub const B8: Square = Square(1);
    pub const C8: Square = Square(2);
    pub const D8: Square = Square(3);
    pub const E8: Square = Square(4);
    pub const F8: Square = Square(5);
    pub const G8: Square = Square(6);
    pub const H8: Square = Square(7);
    pub const A7: Square = Square(8);
    pub const B7: Square = Square(9);
    pub const C7: Square = Square(10);
    pub const D7: Square = Square(11);
    pub const E7: Square = Square(12);
    pub const F7: Square = Square(13);
    pub const G7: Square = Square(14);
    pub const H7: Square = Square(15);
    pub const A6: Square = Square(16);
    pub const B6: Square = Square(17);
    pub const C6: Square = Square(18);
    pub const D6: Square = Square(19);
    pub const E6: Square = Square(20);
    pub const F6: Square = Square(21);
    pub const G6: Square = Square(22);
    pub const H6: Square = Square(23);
    pub const A5: Square = Square(24);
    pub const B5: Square = Square(25);
    pub const C5: Square = Square(26);
    pub const D5: Square = Square(27);
    pub const E5: Square = Square(28);
    pub const F5: Square = Square(29);
    pub const G5: Square = Square(30);
    pub const H5: Square = Square(31);
    pub const A4: Square = Square(32);
    pub const B4: Square = Square(33);
    pub const C4: Square = Square(34);
    pub const D4: Square = Square(35);
    pub const E4: Square = Square(36);
    pub const F4: Square = Square(37);
    pub const G4: Square = Square(38);
    pub const H4: Square = Square(39);
    pub const A3: Square = Square(40);
    pub const B3: Square = Square(41);
    pub const C3: Square = Square(42);
    pub const D3: Square = Square(43);
    pub const E3: Square = Square(44);
    pub const F3: Square = Square(45);
    pub const G3: Square = Square(46);
    pub const H3: Square = Square(47);
    pub const A2: Square = Square(48);
    pub const B2: Square = Square(49);
    pub const C2: Square = Square(50);
    pub const D2: Square = Square(51);
    pub const E2: Square = Square(52);
    pub const F2: Square = Square(53);
    pub const G2: Square = Square(54);
    pub const H2: Square = Square(55);
    pub const A1: Square = Square(56);
    pub const B1: Square = Square(57);
    pub const C1: Square = Square(58);
    pub const D1: Square = Square(59);
    pub const E1: Square = Square(60);
    pub const F1: Square = Square(61);
    pub const G1: Square = Square(62);
    pub const H1: Square = Square(63);
}

impl Square {
    pub fn to_repr(self) -> u8 {
        self.0
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    pub fn from_repr(repr: u8) -> Option<Self> {
        if repr < 64 {
            Some(Square(repr))
        } else {
            None
        }
    }

    pub fn from_repr_unchecked(repr: u8) -> Self {
        Square(repr)
    }

    pub fn from_xy(x: u8, y: u8) -> Option<Self> {
        let index = x + y * 8;

        Self::from_repr(index)
    }

    pub fn from_xy_unchecked(x: u8, y: u8) -> Self {
        let index = x + y * 8;

        Self::from_repr_unchecked(index)
    }

    pub const fn to_bb(self) -> Bitboard {
        Bitboard(1 << self.0)
    }

    pub fn to_xy(self) -> (u8, u8) {
        let x = self.0 % 8;
        let y = self.0 / 8;

        (x, y)
    }
}

impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<_> = s.to_lowercase().chars().collect();

        if chars.len() != 2 {
            return Err(format!("Invalid square: {}", s));
        }

        let file = chars[0];
        let rank = chars[1];

        if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
            return Err(format!("Invalid square: {}", s));
        }

        let x = file as u8 - b'a';
        let y = 7 - (rank as u8 - b'1');

        Ok(Square::from_xy_unchecked(x, y))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.to_xy();

        let file = (b'a' + x) as char;
        let rank = (b'1' + (7 - y)) as char;

        write!(f, "{}{}", file, rank)
    }
}

impl Add<u8> for Square {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Square(self.0 + rhs)
    }
}

impl Sub<u8> for Square {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Square(self.0 - rhs)
    }
}

pub const FILE_A: Bitboard = Bitboard(0x0101010101010101);
pub const FILE_B: Bitboard = FILE_A.shl(1);
pub const FILE_G: Bitboard = FILE_A.shl(6);
pub const FILE_H: Bitboard = FILE_A.shl(7);

pub const NOT_FILE_A: Bitboard = FILE_A.not();
pub const NOT_FILE_H: Bitboard = FILE_H.not();
pub const NOT_FILE_AB: Bitboard = (FILE_A.bitor(FILE_B)).not();
pub const NOT_FILE_GH: Bitboard = (FILE_G.bitor(FILE_H)).not();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_to_repr() {
        assert_eq!(Square::A8.to_repr(), 0);
        assert_eq!(Square::H8.to_repr(), 7);
        assert_eq!(Square::A1.to_repr(), 56);
        assert_eq!(Square::H1.to_repr(), 63);
    }

    #[test]
    fn test_square_from_repr() {
        assert_eq!(Square::from_repr(0), Some(Square::A8));
        assert_eq!(Square::from_repr(7), Some(Square::H8));
        assert_eq!(Square::from_repr(56), Some(Square::A1));
        assert_eq!(Square::from_repr(63), Some(Square::H1));
        assert_eq!(Square::from_repr(64), None);
    }

    #[test]
    fn test_file_masks() {
        assert_eq!(FILE_A & Square::A8.to_bb(), Square::A8.to_bb());
        assert_eq!(FILE_A & Square::B8.to_bb(), Bitboard::empty());
        assert_eq!(FILE_A & FILE_B, Bitboard::empty());

        assert_eq!(FILE_H & Square::H8.to_bb(), Square::H8.to_bb());
        assert_eq!(FILE_H & Square::G8.to_bb(), Bitboard::empty());
        assert_eq!(FILE_H & FILE_G, Bitboard::empty());

        assert_eq!(NOT_FILE_A & Square::A8.to_bb(), Bitboard::empty());
        assert_eq!(NOT_FILE_A & Square::B8.to_bb(), Square::B8.to_bb());
        assert_eq!(NOT_FILE_A & FILE_B, FILE_B);

        assert_eq!(NOT_FILE_H & Square::H8.to_bb(), Bitboard::empty());
        assert_eq!(NOT_FILE_H & Square::G8.to_bb(), Square::G8.to_bb());
        assert_eq!(NOT_FILE_H & FILE_G, FILE_G);

        assert_eq!(NOT_FILE_AB & Square::A8.to_bb(), Bitboard::empty());
        assert_eq!(NOT_FILE_AB & Square::B8.to_bb(), Bitboard::empty());
        assert_eq!(NOT_FILE_AB & Square::C8.to_bb(), Square::C8.to_bb());

        assert_eq!(NOT_FILE_GH & Square::H8.to_bb(), Bitboard::empty());
        assert_eq!(NOT_FILE_GH & Square::G8.to_bb(), Bitboard::empty());
        assert_eq!(NOT_FILE_GH & Square::F8.to_bb(), Square::F8.to_bb());
    }
}
