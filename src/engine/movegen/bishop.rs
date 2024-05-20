use crate::engine::{
    bitboard::{Bitboard, Square},
    color::Color,
    movegen::{
        magic::{
            calculate_magic_index, generate_magic_number_candidate, ARRAY_SIZE_BISHOP,
            MAX_ATTEMPTS, PRECOMPUTED_MAGIC_NUMBERS,
        },
        occupancy::mask_occupancy,
    },
    moves::{Move, MoveFlags},
    piece::Piece,
    position::Position,
};

#[derive(Clone)]
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
        let magic_numbers = PRECOMPUTED_MAGIC_NUMBERS.bishop;
        let attack_table = Self::generate_attack_table(&attack_candidate_table, &magic_numbers);

        Self {
            attack_candidate_table,
            relevant_bits_table,
            magic_numbers,
            attack_table,
        }
    }

    pub fn fresh() -> Self {
        let attack_candidate_table = Self::generate_attack_candidate_table();
        let relevant_bits_table = Self::generate_relevant_bits_table(&attack_candidate_table);
        let magic_numbers = Self::generate_magic_numbers();
        let attack_table = Self::generate_attack_table(&attack_candidate_table, &magic_numbers);

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

        let (square_x, square_y) = square.to_xy_i8();

        let (mut x, mut y) = (square_x + 1, square_y + 1);
        while (x < 7) && (y < 7) {
            attacks.set(Square::from_xy_i8(x, y).unwrap());
            x += 1;
            y += 1;
        }

        let (mut x, mut y) = (square_x - 1, square_y - 1);
        while (x > 0) && (y > 0) {
            attacks.set(Square::from_xy_i8(x, y).unwrap());
            x -= 1;
            y -= 1;
        }

        let (mut x, mut y) = (square_x + 1, square_y - 1);
        while (x < 7) && (y > 0) {
            attacks.set(Square::from_xy_i8(x, y).unwrap());
            x += 1;
            y -= 1;
        }

        let (mut x, mut y) = (square_x - 1, square_y + 1);
        while (x > 0) && (y < 7) {
            attacks.set(Square::from_xy_i8(x, y).unwrap());
            x -= 1;
            y += 1;
        }

        attacks
    }

    fn mask_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::empty();

        let (square_x, square_y) = square.to_xy_i8();

        let (mut x, mut y) = (square_x + 1, square_y + 1);
        while (x <= 7) && (y <= 7) {
            let s = Square::from_xy_i8(x, y).unwrap();
            attacks.set(s);

            if occupancy.get(s) {
                break;
            }

            x += 1;
            y += 1;
        }

        let (mut x, mut y) = (square_x - 1, square_y - 1);
        while (x >= 0) && (y >= 0) {
            let s = Square::from_xy_i8(x, y).unwrap();
            attacks.set(s);

            if occupancy.get(s) {
                break;
            }

            x -= 1;
            y -= 1;
        }

        let (mut x, mut y) = (square_x + 1, square_y - 1);
        while (x <= 7) && (y >= 0) {
            let s = Square::from_xy_i8(x, y).unwrap();
            attacks.set(s);

            if occupancy.get(s) {
                break;
            }

            x += 1;
            y -= 1;
        }

        let (mut x, mut y) = (square_x - 1, square_y + 1);
        while (x >= 0) && (y <= 7) {
            let s = Square::from_xy_i8(x, y).unwrap();
            attacks.set(s);

            if occupancy.get(s) {
                break;
            }

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
        let mut magic_numbers = [0; 64];

        for square in Bitboard::all_squares() {
            magic_numbers[square.to_usize()] = Self::generate_magic_number(square);
        }

        magic_numbers
    }

    fn generate_magic_number(square: Square) -> u64 {
        let mut occupancies = [Bitboard::empty(); ARRAY_SIZE_BISHOP];
        let mut attacks = [Bitboard::empty(); ARRAY_SIZE_BISHOP];

        let candidate_mask = Self::mask_attack_candidates(square);
        let bits_in_mask = candidate_mask.count_ones();
        let index_upper_bound = 1usize << bits_in_mask;

        for index in 0..index_upper_bound {
            occupancies[index] = mask_occupancy(candidate_mask, index);
            attacks[index] = Self::mask_attacks(square, occupancies[index]);
        }

        'outer: for _ in 0..MAX_ATTEMPTS {
            let candidate = generate_magic_number_candidate();

            if (candidate_mask.to_repr().wrapping_mul(candidate) & 0xFF00_0000_0000_0000)
                .count_ones()
                < 6
            {
                continue;
            }

            let mut used_attacks = [Bitboard::empty(); ARRAY_SIZE_BISHOP];

            for index in 0..index_upper_bound {
                let magic_index =
                    calculate_magic_index(occupancies[index], candidate, bits_in_mask);

                if used_attacks[magic_index].is_empty() {
                    used_attacks[magic_index] = attacks[index];
                } else if used_attacks[magic_index] != attacks[index] {
                    continue 'outer;
                }
            }

            return candidate;
        }

        panic!("Failed to generate magic number for square {}", square);
    }

    fn generate_attack_table(
        candidate_table: &[Bitboard; 64],
        magic_numbers: &[u64; 64],
    ) -> Vec<Vec<Bitboard>> {
        let mut table = vec![vec![]; 64];

        for square in Bitboard::all_squares() {
            let candidate_mask = candidate_table[square.to_usize()];
            let bits_in_mask = candidate_mask.count_ones();
            let index_upper_bound = 1usize << bits_in_mask;

            table[square.to_usize()] = vec![Bitboard::empty(); index_upper_bound];

            for index in 0..index_upper_bound {
                let occupancy = mask_occupancy(candidate_mask, index);
                let magic_index = calculate_magic_index(
                    occupancy,
                    magic_numbers[square.to_usize()],
                    bits_in_mask,
                );

                table[square.to_usize()][magic_index] = Self::mask_attacks(square, occupancy);
            }
        }

        table
    }

    pub(super) fn get_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let masked_occupancy = occupancy & self.attack_candidate_table[square.to_usize()];
        let magic_number = self.magic_numbers[square.to_usize()];
        let relevant_bits = self.relevant_bits_table[square.to_usize()];

        let magic_index = calculate_magic_index(masked_occupancy, magic_number, relevant_bits);

        self.attack_table[square.to_usize()][magic_index]
    }

    pub fn generate_pseudo_legal_moves(&self, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => Piece::WhiteBishop,
            Color::Black => Piece::BlackBishop,
        };

        let mut moves = vec![];

        let bishops = position.bitboards().piece(piece);

        for from_square in bishops.squares() {
            let attacks = self.get_attacks(from_square, position.bitboards().all())
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

    pub fn magic_numbers(&self) -> [u64; 64] {
        self.magic_numbers
    }
}

impl Default for BishopMoveGen {
    fn default() -> Self {
        Self::new()
    }
}
