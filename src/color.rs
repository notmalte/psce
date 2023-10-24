#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn to_string(self) -> String {
        match self {
            Self::White => "White",
            Self::Black => "Black",
        }
        .to_string()
    }

    pub fn opponent(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
