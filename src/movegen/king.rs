use crate::{
    bitboard::set_bit,
    constants::{NOT_FILE_A, NOT_FILE_H},
};

pub struct KingMoveGen {
    attack_table: [u64; 64],
}

impl KingMoveGen {
    fn generate_attack_mask(square: u8) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = 0u64;

        set_bit(&mut bitboard, square);

        attacks |= (bitboard >> 9) & NOT_FILE_H;
        attacks |= bitboard >> 8;
        attacks |= (bitboard >> 7) & NOT_FILE_A;
        attacks |= (bitboard >> 1) & NOT_FILE_H;
        attacks |= (bitboard << 1) & NOT_FILE_A;
        attacks |= (bitboard << 7) & NOT_FILE_H;
        attacks |= bitboard << 8;
        attacks |= (bitboard << 9) & NOT_FILE_A;

        attacks
    }

    fn generate_attack_table() -> [u64; 64] {
        let mut table = [0; 64];

        for square in 0..64u8 {
            table[square as usize] = Self::generate_attack_mask(square);
        }

        table
    }

    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }
}
