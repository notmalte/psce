use std::fmt::Display;

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

    pub fn find_piece_on_square(&self, square: u8) -> Option<Piece> {
        for (piece, bitboard) in self.pieces.iter().enumerate() {
            if bitboard.get(square) {
                return Some(Piece::from_repr(piece as u8).unwrap());
            }
        }

        None
    }

    pub fn find_piece_on_xy(&self, x: u8, y: u8) -> Option<Piece> {
        self.find_piece_on_square(Bitboard::xy_to_index(x, y))
    }
}

impl Display for BitboardContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  a b c d e f g h")?;

        for y in 0..8 {
            write!(f, "{} ", 8 - y)?;

            for x in 0..8 {
                if x != 0 {
                    write!(f, " ")?;
                }

                if let Some(piece) = self.find_piece_on_xy(x, y) {
                    write!(f, "{}", piece)?;
                } else {
                    write!(f, ".")?;
                }
            }

            if y != 7 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
