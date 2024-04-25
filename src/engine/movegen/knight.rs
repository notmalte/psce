use crate::engine::{
    bitboard::{Bitboard, NOT_FILE_A, NOT_FILE_AB, NOT_FILE_GH, NOT_FILE_H},
    color::Color,
    moves::{Move, MoveFlags},
    piece::Piece,
    position::Position,
};

pub struct KnightMoveGen {
    attack_table: [Bitboard; 64],
}

impl KnightMoveGen {
    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }

    fn generate_attack_table() -> [Bitboard; 64] {
        let mut table = [Bitboard::empty(); 64];

        for square in Bitboard::all_squares() {
            table[square.to_repr() as usize] = Self::mask_attacks(square);
        }

        table
    }

    fn mask_attacks(square: crate::engine::bitboard::Square) -> Bitboard {
        let mut attacks = Bitboard::empty();
        let bb = square.to_bb();

        attacks |= (bb >> 17) & NOT_FILE_H;
        attacks |= (bb >> 15) & NOT_FILE_A;
        attacks |= (bb >> 10) & NOT_FILE_GH;
        attacks |= (bb >> 6) & NOT_FILE_AB;
        attacks |= (bb << 6) & NOT_FILE_GH;
        attacks |= (bb << 10) & NOT_FILE_AB;
        attacks |= (bb << 15) & NOT_FILE_H;
        attacks |= (bb << 17) & NOT_FILE_A;

        attacks
    }

    pub fn generate_moves(&self, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => Piece::WhiteKnight,
            Color::Black => Piece::BlackKnight,
        };

        let mut moves = vec![];

        let knights = position.bitboards().piece(piece);

        for from_square in knights.squares() {
            let attacks = self.attack_table[from_square.to_repr() as usize]
                & !position.bitboards().color(color);

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

        moves
    }
}
