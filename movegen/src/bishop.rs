use psce_core::{Bitboard, Move, MoveFlags, Piece, Position};
use psce_movegen_generated::{
    BISHOP_ATTACKS, BISHOP_MAGICS, BISHOP_MASKS, BISHOP_OFFSETS, BISHOP_SHIFTS,
};

use crate::{MoveGen, magic::magic_index};

impl MoveGen {
    pub(crate) fn bishop_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
        let masked = occupancy & Bitboard::new(BISHOP_MASKS[square as usize]);
        let magic_number = BISHOP_MAGICS[square as usize];
        let shift = BISHOP_SHIFTS[square as usize];
        let offset = BISHOP_OFFSETS[square as usize];

        let index = magic_index(masked, magic_number, shift) + offset;

        Bitboard::new(BISHOP_ATTACKS[index])
    }

    pub(crate) fn bishop_pseudo_legals(position: &Position, moves: &mut Vec<Move>) {
        let color = position.side_to_move();

        let bishops = position.bitboards().piece(color, Piece::Bishop);
        let all = position.bitboards().all();
        let own = position.bitboards().color(color);

        for from_square in bishops.squares() {
            let attacks = Self::bishop_attacks(from_square, all) & !own;

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
                    Piece::Bishop,
                    None,
                    flags,
                ));
            }
        }
    }
}
