use crate::{
    bitboard::{clear_bit, get_bit, set_bit},
    constants::{B1, B8, C1, C8, D1, D8, E1, E8, F1, F8, G1, G8, NOT_FILE_A, NOT_FILE_H},
    enums::{CastlingRights, Color, PieceAndColor},
    movegen::Move,
    position::Position,
};

use super::MoveGen;

#[derive(Clone)]
pub struct KingMoveGen {
    attack_table: [u64; 64],
}

impl KingMoveGen {
    fn generate_attack_mask(square: u8) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = 0u64;

        set_bit(&mut bitboard, square);

        attacks |= (bitboard >> 9) & NOT_FILE_H;
        attacks |= bitboard >> 8;
        attacks |= (bitboard >> 7) & NOT_FILE_A;
        attacks |= (bitboard >> 1) & NOT_FILE_H;
        attacks |= (bitboard << 1) & NOT_FILE_A;
        attacks |= (bitboard << 7) & NOT_FILE_H;
        attacks |= bitboard << 8;
        attacks |= (bitboard << 9) & NOT_FILE_A;

        attacks
    }

    fn generate_attack_table() -> [u64; 64] {
        let mut table = [0; 64];

        for square in 0..64u8 {
            table[square as usize] = Self::generate_attack_mask(square);
        }

        table
    }

    pub fn attack_table(&self) -> &[u64; 64] {
        &self.attack_table
    }

    // for castling: not checking target square for attacks,
    // as moves resulting in king being in check are filtered out later
    pub fn generate_moves(&self, move_gen: &MoveGen, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => PieceAndColor::WhiteKing,
            Color::Black => PieceAndColor::BlackKing,
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

        match color {
            Color::White => {
                let castle_kingside =
                    position.castling_rights() & CastlingRights::WhiteKingside.flag() != 0;
                let castle_queenside =
                    position.castling_rights() & CastlingRights::WhiteQueenside.flag() != 0;

                let bitboard_both = position.bitboards().color(Color::Both);

                if castle_kingside {
                    if !get_bit(bitboard_both, F1)
                        && !get_bit(bitboard_both, G1)
                        && !move_gen.is_square_attacked(position, E1, Color::Black)
                        && !move_gen.is_square_attacked(position, F1, Color::Black)
                    {
                        moves.push(Move::new(E1, G1, piece, None, false, false, true, false));
                    }
                }

                if castle_queenside {
                    if !get_bit(bitboard_both, D1)
                        && !get_bit(bitboard_both, C1)
                        && !get_bit(bitboard_both, B1)
                        && !move_gen.is_square_attacked(position, E1, Color::Black)
                        && !move_gen.is_square_attacked(position, D1, Color::Black)
                    {
                        moves.push(Move::new(E1, C1, piece, None, false, false, true, false));
                    }
                }
            }
            Color::Black => {
                let castle_kingside =
                    position.castling_rights() & CastlingRights::BlackKingside.flag() != 0;
                let castle_queenside =
                    position.castling_rights() & CastlingRights::BlackQueenside.flag() != 0;

                let bitboard_both = position.bitboards().color(Color::Both);

                if castle_kingside {
                    if !get_bit(bitboard_both, F8)
                        && !get_bit(bitboard_both, G8)
                        && !move_gen.is_square_attacked(position, E8, Color::White)
                        && !move_gen.is_square_attacked(position, F8, Color::White)
                    {
                        moves.push(Move::new(E8, G8, piece, None, false, false, true, false));
                    }
                }

                if castle_queenside {
                    if !get_bit(bitboard_both, D8)
                        && !get_bit(bitboard_both, C8)
                        && !get_bit(bitboard_both, B8)
                        && !move_gen.is_square_attacked(position, E8, Color::White)
                        && !move_gen.is_square_attacked(position, D8, Color::White)
                    {
                        moves.push(Move::new(E8, C8, piece, None, false, false, true, false));
                    }
                }
            }
            _ => unreachable!(),
        }

        moves
    }

    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }
}
