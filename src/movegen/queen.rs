use crate::{
    bitboard::{clear_bit, get_bit},
    enums::{Color, PieceAndColor},
    position::Position,
};

use super::{BishopMoveGen, Move, RookMoveGen};

pub struct QueenMoveGen {
    rook: RookMoveGen,
    bishop: BishopMoveGen,
}

impl QueenMoveGen {
    pub fn new(rook: RookMoveGen, bishop: BishopMoveGen) -> Self {
        Self { rook, bishop }
    }

    pub fn get_attacks(&self, square: u8, occupancy: u64) -> u64 {
        self.rook.get_attacks(square, occupancy) | self.bishop.get_attacks(square, occupancy)
    }

    pub fn generate_moves(&self, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => PieceAndColor::WhiteQueen,
            Color::Black => PieceAndColor::BlackQueen,
            _ => unreachable!(),
        };

        let mut moves = Vec::new();

        let mut bitboard = position.bitboards().piece(piece);

        while bitboard != 0 {
            let source_square = bitboard.trailing_zeros() as u8;

            let mut attacks = self
                .get_attacks(source_square, position.bitboards().color(Color::Both))
                & !position.bitboards().color(color);

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
}
