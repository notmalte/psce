use crate::engine::{bitboard::Bitboard, color::Color, piece::Piece};

pub struct BitboardContainer {
    pieces: [Bitboard; 12],
    colors: [Bitboard; 2],
    all: Bitboard,
}

impl BitboardContainer {
    pub fn empty() -> Self {
        Self {
            pieces: [Bitboard::empty(); 12],
            colors: [Bitboard::empty(); 2],
            all: Bitboard::empty(),
        }
    }

    pub fn piece(&self, piece: Piece) -> Bitboard {
        self.pieces[piece as usize]
    }

    pub fn piece_mut(&mut self, piece: Piece) -> &mut Bitboard {
        &mut self.pieces[piece as usize]
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
