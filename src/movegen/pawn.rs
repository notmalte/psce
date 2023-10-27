use crate::{
    bitboard::set_bit,
    constants::{NOT_FILE_A, NOT_FILE_H},
    enums::Color,
};

pub struct PawnMoveGen {
    attack_table: [[u64; 64]; 2],
}

impl PawnMoveGen {
    fn mask_attacks(color: Color, square: u8) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = 0u64;

        set_bit(&mut bitboard, square);

        if color == Color::White {
            attacks |= (bitboard >> 7) & NOT_FILE_A;
            attacks |= (bitboard >> 9) & NOT_FILE_H;
        } else {
            attacks |= (bitboard << 7) & NOT_FILE_H;
            attacks |= (bitboard << 9) & NOT_FILE_A;
        }

        attacks
    }

    fn generate_attack_table() -> [[u64; 64]; 2] {
        let mut table = [[0; 64]; 2];

        for square in 0..64u8 {
            table[Color::White as usize][square as usize] =
                Self::mask_attacks(Color::White, square);
            table[Color::Black as usize][square as usize] =
                Self::mask_attacks(Color::Black, square);
        }

        table
    }

    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }
}
