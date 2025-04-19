use strum::EnumIter;

#[derive(Copy, Clone, PartialEq, Debug, EnumIter)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
