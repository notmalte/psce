use crate::engine::{
    bitboard::{Bitboard, Square, NOT_FILE_A, NOT_FILE_H},
    color::Color,
};

pub struct PawnMoveGen {
    attack_table: [[Bitboard; 64]; 2],
}

impl PawnMoveGen {
    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }

    fn generate_attack_table() -> [[Bitboard; 64]; 2] {
        let mut table = [[Bitboard::empty(); 64]; 2];

        for square in Bitboard::squares() {
            table[Color::White.to_repr() as usize][square.to_repr() as usize] =
                Self::mask_attacks(Color::White, square);
            table[Color::Black.to_repr() as usize][square.to_repr() as usize] =
                Self::mask_attacks(Color::Black, square);
        }

        table
    }

    fn mask_attacks(color: Color, square: Square) -> Bitboard {
        let mut attacks = Bitboard::empty();
        let bb = square.to_bb();

        if color == Color::White {
            attacks |= (bb >> 7) & NOT_FILE_A;
            attacks |= (bb >> 9) & NOT_FILE_H;
        } else {
            attacks |= (bb << 7) & NOT_FILE_H;
            attacks |= (bb << 9) & NOT_FILE_A;
        }

        attacks
    }
}
