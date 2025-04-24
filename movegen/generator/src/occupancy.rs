use core::Bitboard;

pub(crate) fn mask_occupancy(attack_mask: Bitboard, mut index: usize) -> Bitboard {
    let mut occupancy = Bitboard::empty();

    for square in attack_mask.squares() {
        if index & 1 != 0 {
            occupancy.set(square);
        }

        index >>= 1;
    }

    occupancy
}
