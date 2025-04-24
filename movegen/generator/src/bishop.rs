use core::{Bitboard, Square};

use crate::magic::{MagicData, generate_magic_data};

fn generate_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard::empty(); 64];

    for square in Bitboard::all_squares() {
        let (rank, file) = Square::to_rf(square);

        let mut mask = Bitboard::empty();

        for (r, f) in (rank + 1..7).zip(file + 1..7) {
            mask.set(Square::from_rf(r, f));
        }

        for (r, f) in (1..rank).rev().zip((1..file).rev()) {
            mask.set(Square::from_rf(r, f));
        }

        for (r, f) in (rank + 1..7).zip((1..file).rev()) {
            mask.set(Square::from_rf(r, f));
        }

        for (r, f) in (1..rank).rev().zip(file + 1..7) {
            mask.set(Square::from_rf(r, f));
        }

        masks[square as usize] = mask;
    }

    masks
}

fn generate_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::empty();

    let (rank, file) = Square::to_rf(square);

    for (r, f) in (rank + 1..8).zip(file + 1..8) {
        let s = Square::from_rf(r, f);
        attacks.set(s);

        if occupancy.get(s) {
            break;
        }
    }

    for (r, f) in (0..rank).rev().zip((0..file).rev()) {
        let s = Square::from_rf(r, f);
        attacks.set(s);

        if occupancy.get(s) {
            break;
        }
    }

    for (r, f) in (rank + 1..8).zip((0..file).rev()) {
        let s = Square::from_rf(r, f);
        attacks.set(s);

        if occupancy.get(s) {
            break;
        }
    }

    for (r, f) in (0..rank).rev().zip(file + 1..8) {
        let s = Square::from_rf(r, f);
        attacks.set(s);

        if occupancy.get(s) {
            break;
        }
    }

    attacks
}

pub fn generate_bishop_magic_data() -> MagicData {
    generate_magic_data(generate_masks, generate_attacks)
}
