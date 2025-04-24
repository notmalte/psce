use core::{Bitboard, Square};

use crate::{MagicData, occupancy::mask_occupancy, random_magic};

fn generate_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard::empty(); 64];

    for square in Bitboard::all_squares() {
        let (rank, file) = Square::to_rf(square);

        let mut mask = Bitboard::empty();

        for i in 1..7 {
            mask.set(Square::from_rf(i, file));
            mask.set(Square::from_rf(rank, i));
        }

        mask.clear(square);

        masks[square as usize] = mask;
    }

    masks
}

fn generate_shifts(masks: &[Bitboard; 64]) -> [u8; 64] {
    let mut shifts = [0; 64];

    for square in Bitboard::all_squares() {
        shifts[square as usize] = 64 - masks[square as usize].count();
    }

    shifts
}

fn generate_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::empty();

    let (rank, file) = Square::to_rf(square);

    for r in (rank + 1)..8 {
        let s = Square::from_rf(r, file);
        attacks.set(s);

        if occupancy.get(s) {
            break;
        }
    }

    for r in (0..rank).rev() {
        let s = Square::from_rf(r, file);
        attacks.set(s);

        if occupancy.get(s) {
            break;
        }
    }

    for f in (file + 1)..8 {
        let s = Square::from_rf(rank, f);
        attacks.set(s);

        if occupancy.get(s) {
            break;
        }
    }

    for f in (0..file).rev() {
        let s = Square::from_rf(rank, f);
        attacks.set(s);

        if occupancy.get(s) {
            break;
        }
    }

    attacks
}

fn generate_magic_and_attacks(square: u8, mask: Bitboard, shift: u8) -> (u64, Vec<Bitboard>) {
    let entries = 1 << mask.count();

    let mut occupancies = vec![Bitboard::empty(); entries];
    let mut attacks = vec![Bitboard::empty(); entries];

    for index in 0..entries {
        let occupancy = mask_occupancy(mask, index);

        occupancies[index] = occupancy;
        attacks[index] = generate_attacks(square, occupancy);
    }

    'search: loop {
        let magic = random_magic();

        if (mask.to_repr().wrapping_mul(magic) & 0xFF00_0000_0000_0000).count_ones() < 6 {
            continue;
        }

        let mut used = vec![None; entries];

        for index in 0..entries {
            let occupancy = occupancies[index];
            let attack = attacks[index];

            let index = (occupancy.to_repr().wrapping_mul(magic) >> shift) as usize;

            if let Some(used_attack) = used[index] {
                if used_attack != attack {
                    continue 'search;
                }
            } else {
                used[index] = Some(attack);
            }
        }

        return (magic, attacks);
    }
}

pub fn generate_rook_magic_data() -> MagicData {
    let masks = generate_masks();
    let shifts = generate_shifts(&masks);

    let mut magics = [0; 64];
    let mut offsets = [0; 64];
    let mut attacks = Vec::new();

    for square in Bitboard::all_squares() {
        let (m, a) =
            generate_magic_and_attacks(square, masks[square as usize], shifts[square as usize]);

        magics[square as usize] = m;
        offsets[square as usize] = attacks.len();
        attacks.extend(a);
    }

    MagicData {
        masks: masks.map(|mask| mask.to_repr()),
        shifts,
        magics,
        offsets,
        attacks: attacks.iter().map(|attack| attack.to_repr()).collect(),
    }
}
