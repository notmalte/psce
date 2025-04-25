use core::{Bitboard, Move, MoveFlags, Piece, Position};

use crate::MoveGen;

impl MoveGen {
    pub(crate) fn queen_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
        Self::bishop_attacks(square, occupancy) | Self::rook_attacks(square, occupancy)
    }

    pub(crate) fn queen_pseudo_legals(position: &Position, moves: &mut Vec<Move>) {
        let color = position.side_to_move();

        let queens = position.bitboards().piece(color, Piece::Queen);
        let all = position.bitboards().all();
        let own = position.bitboards().color(color);

        for from_square in queens.squares() {
            let attacks = Self::queen_attacks(from_square, all) & !own;

            for to_square in attacks.squares() {
                let capture = all.get(to_square);

                let flags = if capture {
                    MoveFlags::CAPTURE
                } else {
                    MoveFlags::NONE
                };

                moves.push(Move::new(from_square, to_square, Piece::Queen, None, flags));
            }
        }
    }
}
