use core::{Bitboard, Move, MoveFlags, Piece, Position};

use generated::{ROOK_ATTACKS, ROOK_MAGICS, ROOK_MASKS, ROOK_OFFSETS, ROOK_SHIFTS};

use crate::{MoveGen, magic::magic_index};

impl MoveGen {
    pub(crate) fn rook_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
        let masked = occupancy & Bitboard::new(ROOK_MASKS[square as usize]);
        let magic_number = ROOK_MAGICS[square as usize];
        let shift = ROOK_SHIFTS[square as usize];
        let offset = ROOK_OFFSETS[square as usize];

        let index = magic_index(masked, magic_number, shift) + offset;

        Bitboard::new(ROOK_ATTACKS[index])
    }

    pub(crate) fn rook_pseudo_legals(position: &Position, moves: &mut Vec<Move>) {
        let color = position.side_to_move();

        let rooks = position.bitboards().piece(color, Piece::Rook);
        let all = position.bitboards().all();
        let own = position.bitboards().color(color);

        for from_square in rooks.squares() {
            let attacks = Self::rook_attacks(from_square, all) & !own;

            for to_square in attacks.squares() {
                let capture = all.get(to_square);

                let flags = if capture {
                    MoveFlags::CAPTURE
                } else {
                    MoveFlags::NONE
                };

                moves.push(Move::new(from_square, to_square, Piece::Rook, None, flags));
            }
        }
    }
}
