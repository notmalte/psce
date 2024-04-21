#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum CastlingRights {
    WhiteKingSide = 0b0001,
    WhiteQueenSide = 0b0010,
    BlackKingSide = 0b0100,
    BlackQueenSide = 0b1000,
    None = 0b0000,
    All = 0b1111,
}

impl CastlingRights {
    pub fn to_repr(self) -> u8 {
        self as u8
    }
}
