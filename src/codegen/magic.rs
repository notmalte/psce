use crate::{
    enums::RookOrBishop,
    movegen::{mask_occupancy, BishopMoveGen, RookMoveGen},
};

fn generate_magic_number_candidate() -> u64 {
    let mut rng = fastrand::Rng::new();

    let n1: u64 = rng.u64(..);
    let n2: u64 = rng.u64(..);
    let n3: u64 = rng.u64(..);

    n1 & n2 & n3
}

fn find_magic_number(square: u8, relevant_bits: u8, rook_or_bishop: RookOrBishop) -> u64 {
    let mut occupancies = [0u64; 4096];

    let mut attacks = [0u64; 4096];

    let candidate_mask = match rook_or_bishop {
        RookOrBishop::Rook => RookMoveGen::generate_candidate_mask(square),
        RookOrBishop::Bishop => BishopMoveGen::generate_candidate_mask(square),
    };

    let occupancy_indices = 1u64 << relevant_bits;

    for index in 0..occupancy_indices {
        occupancies[index as usize] = mask_occupancy(index, relevant_bits, candidate_mask);

        attacks[index as usize] = match rook_or_bishop {
            RookOrBishop::Rook => {
                RookMoveGen::generate_attack_mask(square, occupancies[index as usize])
            }
            RookOrBishop::Bishop => {
                BishopMoveGen::generate_attack_mask(square, occupancies[index as usize])
            }
        }
    }

    'outer: for _ in 0..1_000_000_000 {
        let candidate = generate_magic_number_candidate();

        if ((candidate_mask.wrapping_mul(candidate)) & 0xFF00_0000_0000_0000).count_ones() < 6 {
            continue;
        }

        let mut used_attacks = [0u64; 4096];

        for index in 0..occupancy_indices {
            let magic_index =
                (occupancies[index as usize].wrapping_mul(candidate)) >> (64 - relevant_bits);

            if used_attacks[magic_index as usize] == 0 {
                used_attacks[magic_index as usize] = attacks[index as usize]
            } else if used_attacks[magic_index as usize] != attacks[index as usize] {
                continue 'outer;
            }
        }

        return candidate;
    }

    panic!("Could not find magic number");
}

pub fn generate_and_print_magic_numbers() {
    let rook_relevant_bits = RookMoveGen::generate_relevant_bits_table_directly();
    let bishop_relevant_bits = BishopMoveGen::generate_relevant_bits_table_directly();

    println!("pub const ROOK_MAGIC_NUMBERS: [u64; 64] = [");
    for square in 0..64 {
        println!(
            "    {},",
            find_magic_number(
                square,
                rook_relevant_bits[square as usize],
                RookOrBishop::Rook
            )
        );
    }
    println!("];\n\npub const BISHOP_MAGIC_NUMBERS: [u64; 64] = [");
    for square in 0..64 {
        println!(
            "    {},",
            find_magic_number(
                square,
                bishop_relevant_bits[square as usize],
                RookOrBishop::Bishop
            )
        );
    }
    println!("];");
}
