use core::{Bitboard, Move, MoveFlags, Piece, Position, Square};

use crate::MoveGen;

impl MoveGen {
    pub(crate) fn generate_knight_attacks() -> [Bitboard; 64] {
        let mut attacks = [Bitboard::empty(); 64];

        for square in Bitboard::all_squares() {
            let bb = Square::to_bb(square);

            let mut mask = Bitboard::empty();

            mask |= bb.north().north().east() & Bitboard::NOT_FILE_A;
            mask |= bb.north().east().east() & Bitboard::NOT_FILE_AB;
            mask |= bb.south().east().east() & Bitboard::NOT_FILE_AB;
            mask |= bb.south().south().east() & Bitboard::NOT_FILE_A;
            mask |= bb.south().south().west() & Bitboard::NOT_FILE_H;
            mask |= bb.south().west().west() & Bitboard::NOT_FILE_GH;
            mask |= bb.north().west().west() & Bitboard::NOT_FILE_GH;
            mask |= bb.north().north().west() & Bitboard::NOT_FILE_H;

            attacks[square as usize] = mask;
        }

        attacks
    }

    pub(crate) fn knight_attacks(&self, square: u8) -> Bitboard {
        self.knight_attacks[square as usize]
    }

    pub(crate) fn knight_pseudo_legals(&self, position: &Position, moves: &mut Vec<Move>) {
        let color = position.side_to_move();

        let knights = position.bitboards().piece(color, Piece::Knight);
        let all = position.bitboards().all();
        let own = position.bitboards().color(color);

        for from_square in knights.squares() {
            let attacks = self.knight_attacks(from_square) & !own;

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
