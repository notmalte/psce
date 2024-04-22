use crate::engine::{bitboard::BitboardContainer, castling::CastlingRights, color::Color};

pub struct Position {
    bitboards: BitboardContainer,
    color_to_move: Color,
    castling_rights: CastlingRights,
    en_passant_square: Option<u8>,
}

impl Position {
    pub fn empty() -> Self {
        Self {
            bitboards: BitboardContainer::empty(),
            color_to_move: Color::White,
            castling_rights: CastlingRights::none(),
            en_passant_square: None,
        }
    }
}
