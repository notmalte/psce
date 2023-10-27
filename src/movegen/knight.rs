use crate::{
    bitboard::set_bit,
    constants::{NOT_FILE_A, NOT_FILE_AB, NOT_FILE_GH, NOT_FILE_H},
};

pub struct KnightMoveGen {
    attack_table: [u64; 64],
}

impl KnightMoveGen {
    fn mask_attacks(square: u8) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = 0u64;

        set_bit(&mut bitboard, square);

        attacks |= (bitboard >> 17) & NOT_FILE_H;
        attacks |= (bitboard >> 15) & NOT_FILE_A;
        attacks |= (bitboard >> 10) & NOT_FILE_GH;
        attacks |= (bitboard >> 6) & NOT_FILE_AB;
        attacks |= (bitboard << 6) & NOT_FILE_GH;
        attacks |= (bitboard << 10) & NOT_FILE_AB;
        attacks |= (bitboard << 15) & NOT_FILE_H;
        attacks |= (bitboard << 17) & NOT_FILE_A;

        attacks
    }

    fn generate_attack_table() -> [u64; 64] {
        let mut table = [0; 64];

        for square in 0..64u8 {
            table[square as usize] = Self::mask_attacks(square);
        }

        table
    }

    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }
}
