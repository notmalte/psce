use crate::engine::{
    bitboard::{Bitboard, Square, NOT_FILE_A, NOT_FILE_H},
    castling::Castling,
    color::Color,
    movegen::MoveGen,
    moves::{Move, MoveFlags},
    piece::Piece,
    position::Position,
};

pub struct KingMoveGen {
    attack_table: [Bitboard; 64],
}

impl KingMoveGen {
    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }

    fn generate_attack_table() -> [Bitboard; 64] {
        let mut table = [Bitboard::empty(); 64];

        for square in Bitboard::all_squares() {
            table[square.to_usize()] = Self::mask_attacks(square);
        }

        table
    }

    fn mask_attacks(square: Square) -> Bitboard {
        let mut attacks = Bitboard::empty();
        let bb = square.to_bb();

        attacks |= (bb >> 9) & NOT_FILE_H;
        attacks |= bb >> 8;
        attacks |= (bb >> 7) & NOT_FILE_A;
        attacks |= (bb >> 1) & NOT_FILE_H;
        attacks |= (bb << 1) & NOT_FILE_A;
        attacks |= (bb << 7) & NOT_FILE_H;
        attacks |= bb << 8;
        attacks |= (bb << 9) & NOT_FILE_A;

        attacks
    }

    pub fn generate_moves(&self, position: &Position, move_gen: &MoveGen) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => Piece::WhiteKing,
            Color::Black => Piece::BlackKing,
        };

        let (king_side_castle_flag, queen_side_castle_flag) = match color {
            Color::White => (Castling::WHITE_KING_SIDE, Castling::WHITE_QUEEN_SIDE),
            Color::Black => (Castling::BLACK_KING_SIDE, Castling::BLACK_QUEEN_SIDE),
        };

        let mut moves = vec![];

        let kings = position.bitboards().piece(piece);

        for from_square in kings.squares() {
            let attacks =
                self.attack_table[from_square.to_usize()] & !position.bitboards().color(color);

            for to_square in attacks.squares() {
                let capture = position.bitboards().all().get(to_square);

                let flags = if capture {
                    MoveFlags::CAPTURE
                } else {
                    MoveFlags::NONE
                };

                moves.push(Move::new(from_square, to_square, piece, None, flags));
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

            if (position.bitboards().all() & mask).is_empty()
                && !move_gen.is_attacked(position, e_square, !color)
                && !move_gen.is_attacked(position, f_square, !color)
            {
                moves.push(Move::new(
                    e_square,
                    g_square,
                    piece,
                    None,
                    MoveFlags::CASTLING,
                ));
            }
        }

        if position.castling().can(queen_side_castle_flag) {
            let (mask, c_square, d_square, e_square) = match color {
                Color::White => (
                    Castling::WHITE_QUEEN_SIDE_SQUARES,
                    Square::C1,
                    Square::D1,
                    Square::E1,
                ),
                Color::Black => (
                    Castling::BLACK_QUEEN_SIDE_SQUARES,
                    Square::C8,
                    Square::D8,
                    Square::E8,
                ),
            };

            if (position.bitboards().all() & mask).is_empty()
                && !move_gen.is_attacked(position, e_square, !color)
                && !move_gen.is_attacked(position, d_square, !color)
            {
                moves.push(Move::new(
                    e_square,
                    c_square,
                    piece,
                    None,
                    MoveFlags::CASTLING,
                ));
            }
        }

        moves
    }
}
