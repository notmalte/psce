use crate::{bitboard::rf_to_square_i8, constants::ROOK_MAGIC_NUMBERS};

use super::mask_occupancy;

#[derive(Clone)]
pub struct RookMoveGen {
    candidate_table: [u64; 64],
    relevant_bits_table: [u8; 64],
    attack_table: Vec<Vec<u64>>, // originally [[u64; 4096]; 64] but this exceeds the stack size
}

impl RookMoveGen {
    pub fn candidate_table(&self) -> &[u64; 64] {
        &self.candidate_table
    }

    pub fn relevant_bits_table(&self) -> &[u8; 64] {
        &self.relevant_bits_table
    }

    pub fn attack_table(&self) -> &Vec<Vec<u64>> {
        &self.attack_table
    }

    pub fn generate_candidate_mask(square: u8) -> u64 {
        let mut attacks = 0u64;

        let target_rank = (square / 8) as i8;
        let target_file = (square % 8) as i8;

        for rank in (target_rank + 1)..7 {
            attacks |= 1 << rf_to_square_i8(rank, target_file);
        }

        for rank in (1..(target_rank)).rev() {
            attacks |= 1 << rf_to_square_i8(rank, target_file);
        }

        for file in (target_file + 1)..7 {
            attacks |= 1 << rf_to_square_i8(target_rank, file);
        }

        for file in (1..(target_file)).rev() {
            attacks |= 1 << rf_to_square_i8(target_rank, file);
        }

        attacks
    }

    fn generate_candidate_table() -> [u64; 64] {
        let mut table = [0; 64];

        for square in 0..64u8 {
            table[square as usize] = Self::generate_candidate_mask(square);
        }

        table
    }

    fn generate_relevant_bits_table(candidate_table: [u64; 64]) -> [u8; 64] {
        let mut table = [0; 64];

        for square in 0..64u8 {
            table[square as usize] = candidate_table[square as usize].count_ones() as u8;
        }

        table
    }

    pub fn generate_relevant_bits_table_directly() -> [u8; 64] {
        Self::generate_relevant_bits_table(Self::generate_candidate_table())
    }

    pub fn generate_attack_mask(square: u8, occupancy: u64) -> u64 {
        let mut attack_mask = 0u64;

        let target_rank = (square / 8) as i8;
        let target_file = (square % 8) as i8;

        for rank in (target_rank + 1)..8 {
            let mask = 1 << rf_to_square_i8(rank, target_file);

            attack_mask |= mask;

            if mask & occupancy != 0 {
                break;
            }
        }

        for rank in (0..(target_rank)).rev() {
            let mask = 1 << rf_to_square_i8(rank, target_file);

            attack_mask |= mask;

            if mask & occupancy != 0 {
                break;
            }
        }

        for file in (target_file + 1)..8 {
            let mask = 1 << rf_to_square_i8(target_rank, file);

            attack_mask |= mask;

            if mask & occupancy != 0 {
                break;
            }
        }

        for file in (0..(target_file)).rev() {
            let mask = 1 << rf_to_square_i8(target_rank, file);

            attack_mask |= mask;

            if mask & occupancy != 0 {
                break;
            }
        }

        attack_mask
    }

    fn generate_attack_table(
        candidate_table: [u64; 64],
        relevant_bits_table: [u8; 64],
    ) -> Vec<Vec<u64>> {
        let mut attack_table = vec![vec![0u64; 4096]; 64];

        for square in 0..64u8 {
            let relevant_bits_count = relevant_bits_table[square as usize];

            let occupancy_indices = 1 << relevant_bits_count;

            for index in 0..occupancy_indices {
                let occupancy =
                    mask_occupancy(index, relevant_bits_count, candidate_table[square as usize]);

                let magic_index = (occupancy.wrapping_mul(ROOK_MAGIC_NUMBERS[square as usize]))
                    >> (64 - relevant_bits_count);

                attack_table[square as usize][magic_index as usize] =
                    Self::generate_attack_mask(square, occupancy);
            }
        }

        attack_table
    }

    pub fn get_attacks(&self, square: u8, occupancy: u64) -> u64 {
        let index = ((occupancy & self.candidate_table[square as usize])
            .wrapping_mul(ROOK_MAGIC_NUMBERS[square as usize]))
            >> (64 - self.relevant_bits_table[square as usize]);

        self.attack_table[square as usize][index as usize]
    }

    pub fn new() -> Self {
        let candidate_table = Self::generate_candidate_table();
        let relevant_bits_table = Self::generate_relevant_bits_table(candidate_table);
        let attack_table = Self::generate_attack_table(candidate_table, relevant_bits_table);

        Self {
            candidate_table,
            relevant_bits_table,
            attack_table,
        }
    }
}
