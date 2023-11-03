use crate::{
    bitboard::{clear_bit, get_bit, set_bit},
    constants::{A1, A2, A7, A8, H1, H2, H7, H8, NOT_FILE_A, NOT_FILE_H},
    enums::{Color, PieceAndColor},
    position::Position,
};

use super::Move;

pub struct PawnMoveGen {
    attack_table: [[u64; 64]; 2],
}

impl PawnMoveGen {
    fn mask_attacks(color: Color, square: u8) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = 0u64;

        set_bit(&mut bitboard, square);

        if color == Color::White {
            attacks |= (bitboard >> 7) & NOT_FILE_A;
            attacks |= (bitboard >> 9) & NOT_FILE_H;
        } else {
            attacks |= (bitboard << 7) & NOT_FILE_H;
            attacks |= (bitboard << 9) & NOT_FILE_A;
        }

        attacks
    }

    fn generate_attack_table() -> [[u64; 64]; 2] {
        let mut table = [[0; 64]; 2];

        for square in 0..64u8 {
            table[Color::White as usize][square as usize] =
                Self::mask_attacks(Color::White, square);
            table[Color::Black as usize][square as usize] =
                Self::mask_attacks(Color::Black, square);
        }

        table
    }

    pub fn attack_table(&self) -> &[[u64; 64]; 2] {
        &self.attack_table
    }

    pub fn generate_moves(&self, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => PieceAndColor::WhitePawn,
            Color::Black => PieceAndColor::BlackPawn,
            _ => unreachable!(),
        };

        let mut moves = Vec::new();

        let mut bitboard = position.bitboards().piece(piece);

        while bitboard != 0 {
            let source_square = bitboard.trailing_zeros() as u8;

            let promotion_rank = match color {
                Color::White => A8..=H8,
                Color::Black => A1..=H1,
                _ => unreachable!(),
            };

            let home_rank = match color {
                Color::White => A2..=H2,
                Color::Black => A7..=H7,
                _ => unreachable!(),
            };

            if promotion_rank.contains(&source_square) {
                continue;
            }

            let target_square = match color {
                Color::White => source_square - 8,
                Color::Black => source_square + 8,
                _ => unreachable!(),
            };

            if !get_bit(position.bitboards().color(Color::Both), target_square) {
                if promotion_rank.contains(&target_square) {
                    moves.push(Move::new(
                        source_square,
                        target_square,
                        piece,
                        Some(PieceAndColor::WhiteQueen),
                        false,
                        false,
                        false,
                        false,
                    ));

                    moves.push(Move::new(
                        source_square,
                        target_square,
                        piece,
                        Some(PieceAndColor::WhiteRook),
                        false,
                        false,
                        false,
                        false,
                    ));

                    moves.push(Move::new(
                        source_square,
                        target_square,
                        piece,
                        Some(PieceAndColor::WhiteBishop),
                        false,
                        false,
                        false,
                        false,
                    ));

                    moves.push(Move::new(
                        source_square,
                        target_square,
                        piece,
                        Some(PieceAndColor::WhiteKnight),
                        false,
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

                    if home_rank.contains(&source_square) {
                        let double_target_square = match color {
                            Color::White => source_square - 16,
                            Color::Black => source_square + 16,
                            _ => unreachable!(),
                        };

                        if !get_bit(
                            position.bitboards().color(Color::Both),
                            double_target_square,
                        ) {
                            moves.push(Move::new(
                                source_square,
                                double_target_square,
                                piece,
                                None,
                                false,
                                false,
                                false,
                                true,
                            ));
                        }
                    }
                }
            }

            let attacks_mask = self.attack_table[color as usize][source_square as usize];

            let mut attacks = attacks_mask & position.bitboards().color(!color);

            while attacks != 0 {
                let capture_square = attacks.trailing_zeros() as u8;

                if promotion_rank.contains(&capture_square) {
                    moves.push(Move::new(
                        source_square,
                        capture_square,
                        piece,
                        Some(PieceAndColor::WhiteQueen),
                        true,
                        false,
                        false,
                        false,
                    ));

                    moves.push(Move::new(
                        source_square,
                        capture_square,
                        piece,
                        Some(PieceAndColor::WhiteRook),
                        true,
                        false,
                        false,
                        false,
                    ));

                    moves.push(Move::new(
                        source_square,
                        capture_square,
                        piece,
                        Some(PieceAndColor::WhiteBishop),
                        true,
                        false,
                        false,
                        false,
                    ));

                    moves.push(Move::new(
                        source_square,
                        capture_square,
                        piece,
                        Some(PieceAndColor::WhiteKnight),
                        true,
                        false,
                        false,
                        false,
                    ));
                } else {
                    moves.push(Move::new(
                        source_square,
                        capture_square,
                        piece,
                        None,
                        true,
                        false,
                        false,
                        false,
                    ));
                }

                clear_bit(&mut attacks, capture_square);
            }

            if let Some(en_passant_square) = position.en_passant_square() {
                if get_bit(attacks_mask, en_passant_square) {
                    moves.push(Move::new(
                        source_square,
                        en_passant_square,
                        piece,
                        None,
                        true,
                        true,
                        false,
                        false,
                    ));
                }
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
