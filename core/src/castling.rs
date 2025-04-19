use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Castling(u8);

impl Castling {
    pub const WHITE_KING_SIDE: Self = Castling(0b0001);
    pub const WHITE_QUEEN_SIDE: Self = Castling(0b0010);
    pub const WHITE_ALL: Self = Castling(0b0011);
    pub const BLACK_KING_SIDE: Self = Castling(0b0100);
    pub const BLACK_QUEEN_SIDE: Self = Castling(0b1000);
    pub const BLACK_ALL: Self = Castling(0b1100);
    pub const NONE: Self = Castling(0b0000);
    pub const ALL: Self = Castling(0b1111);

    pub fn can(self, cstl: Self) -> bool {
        (self.0 & cstl.0) != 0
    }

    pub fn set(&mut self, cstl: Self) {
        self.0 |= cstl.0;
    }

    pub fn clear(&mut self, cstl: Self) {
        self.0 &= !cstl.0;
    }
}

impl FromStr for Castling {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cstl = Self::NONE;

        if s == "-" {
            return Ok(cstl);
        }

        for c in s.chars() {
            match c {
                'K' => cstl.set(Self::WHITE_KING_SIDE),
                'Q' => cstl.set(Self::WHITE_QUEEN_SIDE),
                'k' => cstl.set(Self::BLACK_KING_SIDE),
                'q' => cstl.set(Self::BLACK_QUEEN_SIDE),
                _ => return Err(format!("Invalid castling rights: {}", s)),
            }
        }

        Ok(cstl)
    }
}

impl Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Self::NONE {
            write!(f, "-")
        } else {
            let mut s = String::new();

            if self.can(Self::WHITE_KING_SIDE) {
                s.push('K');
            }

            if self.can(Self::WHITE_QUEEN_SIDE) {
                s.push('Q');
            }

            if self.can(Self::BLACK_KING_SIDE) {
                s.push('k');
            }

            if self.can(Self::BLACK_QUEEN_SIDE) {
                s.push('q');
            }

            write!(f, "{}", s)
        }
    }
}
