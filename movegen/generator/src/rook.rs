use psce_core::{Bitboard, Square};

use crate::magic::{MagicData, generate_magic_data};

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

pub fn generate_rook_magic_data() -> MagicData {
    generate_magic_data(generate_masks, generate_attacks)
}
