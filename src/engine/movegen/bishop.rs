use crate::engine::bitboard::{Bitboard, Square};

pub struct BishopMoveGen {
    attack_candidate_table: [Bitboard; 64],
    relevant_bits_table: [u8; 64],
    magic_numbers: [u64; 64],
    attack_table: Vec<Vec<Bitboard>>,
}

impl BishopMoveGen {
    pub fn new() -> Self {
        let attack_candidate_table = Self::generate_attack_candidate_table();
        let relevant_bits_table = Self::generate_relevant_bits_table(&attack_candidate_table);
        let magic_numbers = Self::generate_magic_numbers();
        let attack_table = Self::generate_attack_table();

        Self {
            attack_candidate_table,
            relevant_bits_table,
            magic_numbers,
            attack_table,
        }
    }

    fn generate_attack_candidate_table() -> [Bitboard; 64] {
        let mut table = [Bitboard::empty(); 64];

        for square in Bitboard::all_squares() {
            table[square.to_usize()] = Self::mask_attack_candidates(square);
        }

        table
    }

    fn mask_attack_candidates(square: Square) -> Bitboard {
        let mut attacks = Bitboard::empty();

        let (square_x, square_y) = square.to_xy();

        let (mut x, mut y) = (square_x + 1, square_y + 1);
        while (x < 7) && (y < 7) {
            attacks.set(Square::from_xy(x, y).unwrap());
            x += 1;
            y += 1;
        }

        let (mut x, mut y) = (square_x - 1, square_y - 1);
        while (x > 0) && (y > 0) {
            attacks.set(Square::from_xy(x, y).unwrap());
            x -= 1;
            y -= 1;
        }

        let (mut x, mut y) = (square_x + 1, square_y - 1);
        while (x < 7) && (y > 0) {
            attacks.set(Square::from_xy(x, y).unwrap());
            x += 1;
            y -= 1;
        }

        let (mut x, mut y) = (square_x - 1, square_y + 1);
        while (x > 0) && (y < 7) {
            attacks.set(Square::from_xy(x, y).unwrap());
            x -= 1;
            y += 1;
        }

        attacks
    }

    fn generate_relevant_bits_table(attack_candidate_table: &[Bitboard; 64]) -> [u8; 64] {
        let mut table = [0; 64];

        for square in Bitboard::all_squares() {
            table[square.to_usize()] = attack_candidate_table[square.to_usize()].count_ones();
        }

        table
    }

    fn generate_magic_numbers() -> [u64; 64] {
        todo!()
    }

    fn generate_attack_table() -> Vec<Vec<Bitboard>> {
        todo!()
    }
}
