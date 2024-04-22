use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Castling {
    WhiteKingSide = 0b0001,
    WhiteQueenSide = 0b0010,
    BlackKingSide = 0b0100,
    BlackQueenSide = 0b1000,
    None = 0b0000,
    All = 0b1111,
}

impl Castling {
    fn to_repr(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut cstl = CastlingRights::none();

        if s == "-" {
            return Ok(cstl);
        }

        for c in s.chars() {
            match c {
                'K' => cstl.set(Castling::WhiteKingSide),
                'Q' => cstl.set(Castling::WhiteQueenSide),
                'k' => cstl.set(Castling::BlackKingSide),
                'q' => cstl.set(Castling::BlackQueenSide),
                _ => return Err(format!("Invalid castling rights: {}", s)),
            }
        }

        Ok(cstl)
    }

    pub fn all() -> Self {
        CastlingRights(Castling::All.to_repr())
    }

    pub fn none() -> Self {
        CastlingRights(Castling::None.to_repr())
    }

    pub fn is_all(&self) -> bool {
        self.0 == Castling::All.to_repr()
    }

    pub fn is_none(&self) -> bool {
        self.0 == Castling::None.to_repr()
    }

    pub fn has(&self, cstl: Castling) -> bool {
        (self.0 & cstl.to_repr()) != 0
    }

    pub fn set(&mut self, cstl: Castling) {
        self.0 |= cstl.to_repr();
    }

    pub fn clear(&mut self, cstl: Castling) {
        self.0 &= !cstl.to_repr();
    }
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_none() {
            write!(f, "-")
        } else {
            let mut s = String::new();

            if self.has(Castling::WhiteKingSide) {
                s.push('K');
            }

            if self.has(Castling::WhiteQueenSide) {
                s.push('Q');
            }

            if self.has(Castling::BlackKingSide) {
                s.push('k');
            }

            if self.has(Castling::BlackQueenSide) {
                s.push('q');
            }

            write!(f, "{}", s)
        }
    }
}
