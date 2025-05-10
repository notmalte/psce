use psce_core::{Bitboard, Move, MoveFlags, Piece, Position};
use psce_movegen_generated::KNIGHT_ATTACKS;

use crate::MoveGen;

impl MoveGen {
    pub(crate) fn knight_attacks(square: u8) -> Bitboard {
        Bitboard::new(KNIGHT_ATTACKS[square as usize])
    }

    pub(crate) fn knight_pseudo_legals(position: &Position, moves: &mut Vec<Move>) {
        let color = position.side_to_move();

        let knights = position.bitboards().piece(color, Piece::Knight);
        let all = position.bitboards().all();
        let own = position.bitboards().color(color);

        for from_square in knights.squares() {
            let attacks = Self::knight_attacks(from_square) & !own;

            for to_square in attacks.squares() {
                let capture = all.get(to_square);

                let flags = if capture {
                    MoveFlags::CAPTURE
                } else {
                    MoveFlags::NONE
                };

                moves.push(Move::new(
                    from_square,
                    to_square,
                    Piece::Knight,
                    None,
                    flags,
                ));
            }
        }
    }
}
