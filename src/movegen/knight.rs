use crate::{
    bitboard::{clear_bit, get_bit, set_bit},
    constants::{NOT_FILE_A, NOT_FILE_AB, NOT_FILE_GH, NOT_FILE_H},
    enums::{Color, PieceAndColor},
    position::Position,
};

use super::Move;

pub struct KnightMoveGen {
    attack_table: [u64; 64],
}

impl KnightMoveGen {
    fn mask_attacks(square: u8) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = 0u64;

        set_bit(&mut bitboard, square);

        attacks |= (bitboard >> 17) & NOT_FILE_H;
        attacks |= (bitboard >> 15) & NOT_FILE_A;
        attacks |= (bitboard >> 10) & NOT_FILE_GH;
        attacks |= (bitboard >> 6) & NOT_FILE_AB;
        attacks |= (bitboard << 6) & NOT_FILE_GH;
        attacks |= (bitboard << 10) & NOT_FILE_AB;
        attacks |= (bitboard << 15) & NOT_FILE_H;
        attacks |= (bitboard << 17) & NOT_FILE_A;

        attacks
    }

    fn generate_attack_table() -> [u64; 64] {
        let mut table = [0; 64];

        for square in 0..64u8 {
            table[square as usize] = Self::mask_attacks(square);
        }

        table
    }

    pub fn attack_table(&self) -> &[u64; 64] {
        &self.attack_table
    }

    pub fn generate_moves(&self, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => PieceAndColor::WhiteKnight,
            Color::Black => PieceAndColor::BlackKnight,
            _ => unreachable!(),
        };

        let mut moves = Vec::new();

        let mut bitboard = position.bitboards().piece(piece);

        while bitboard != 0 {
            let source_square = bitboard.trailing_zeros() as u8;

            let mut attacks =
                self.attack_table[source_square as usize] & !position.bitboards().color(color);

            while attacks != 0 {
                let target_square = attacks.trailing_zeros() as u8;

                if get_bit(position.bitboards().color(Color::Both), target_square) {
                    moves.push(Move::new(
                        source_square,
                        target_square,
                        piece,
                        None,
                        true,
                        false,
                        false,
                        false,
                    ));
                } else {
                    moves.push(Move::new(
                        source_square,
                        target_square,
                        piece,
                        None,
                        false,
                        false,
                        false,
                        false,
                    ));
                }

                clear_bit(&mut attacks, target_square);
            }

            clear_bit(&mut bitboard, source_square);
        }

        moves
    }

    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }
}
