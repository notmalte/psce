use crate::{castling::Castling, piece::Piece};

#[derive(Debug)]
pub struct Undo {
    pub captured_piece: Option<Piece>,
    pub previous_castling: Castling,
    pub previous_en_passant_square: Option<u8>,
    pub previous_halfmove_clock: u8,
    // TODO: hash
}
