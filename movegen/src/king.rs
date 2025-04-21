use core::{Bitboard, Castling, Color, Move, MoveFlags, Piece, Position, Square};

use crate::MoveGen;

impl MoveGen {
    pub(crate) fn generate_king_attacks() -> [Bitboard; 64] {
        let mut attacks = [Bitboard::empty(); 64];

        for square in Bitboard::all_squares() {
            let bb = Square::to_bb(square);

            let mut mask = Bitboard::empty();

            mask |= (bb >> 9) & Bitboard::NOT_FILE_H;
            mask |= bb >> 8;
            mask |= (bb >> 7) & Bitboard::NOT_FILE_A;
            mask |= (bb >> 1) & Bitboard::NOT_FILE_H;
            mask |= (bb << 1) & Bitboard::NOT_FILE_A;
            mask |= (bb << 7) & Bitboard::NOT_FILE_H;
            mask |= bb << 8;
            mask |= (bb << 9) & Bitboard::NOT_FILE_A;

            attacks[square as usize] = mask;
        }

        attacks
    }

    pub(crate) fn king_attacks(&self, square: u8) -> Bitboard {
        self.king_attacks[square as usize]
    }

    pub(crate) fn king_pseudo_legals(&self, position: &Position, moves: &mut Vec<Move>) {
        let color = position.side_to_move();

        let (king_side_castle_flag, queen_side_castle_flag) = match color {
            Color::White => (Castling::WHITE_KING_SIDE, Castling::WHITE_QUEEN_SIDE),
            Color::Black => (Castling::BLACK_KING_SIDE, Castling::BLACK_QUEEN_SIDE),
        };

        let kings = position.bitboards().piece(color, Piece::King);
        let all = position.bitboards().all();
        let own = position.bitboards().color(color);

        for from_square in kings.squares() {
            let attacks = self.king_attacks(from_square) & !own;

            for to_square in attacks.squares() {
                let capture = all.get(to_square);

                let flags = if capture {
                    MoveFlags::CAPTURE
                } else {
                    MoveFlags::NONE
                };

                moves.push(Move::new(from_square, to_square, Piece::King, None, flags));
            }
        }

        if position.castling().can(king_side_castle_flag) {
            let (mask, e_square, f_square, g_square) = match color {
                Color::White => (
                    Castling::WHITE_KING_SIDE_SQUARES,
                    Square::E1,
                    Square::F1,
                    Square::G1,
                ),
                Color::Black => (
                    Castling::BLACK_KING_SIDE_SQUARES,
                    Square::E8,
                    Square::F8,
                    Square::G8,
                ),
            };

            if (all & mask).is_empty()
                && !self.is_attacked(position, e_square, !color)
                && !self.is_attacked(position, f_square, !color)
            {
                moves.push(Move::new(
                    e_square,
                    g_square,
                    Piece::King,
                    None,
                    MoveFlags::CASTLING,
                ));
            }
        }

        if position.castling().can(queen_side_castle_flag) {
            let (mask, e_square, d_square, c_square) = match color {
                Color::White => (
                    Castling::WHITE_QUEEN_SIDE_SQUARES,
                    Square::E1,
                    Square::D1,
                    Square::C1,
                ),
                Color::Black => (
                    Castling::BLACK_QUEEN_SIDE_SQUARES,
                    Square::E8,
                    Square::D8,
                    Square::C8,
                ),
            };

            if (all & mask).is_empty()
                && !self.is_attacked(position, e_square, !color)
                && !self.is_attacked(position, d_square, !color)
            {
                moves.push(Move::new(
                    e_square,
                    c_square,
                    Piece::King,
                    None,
                    MoveFlags::CASTLING,
                ));
            }
        }
    }
}
