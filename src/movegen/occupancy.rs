use crate::bitboard::clear_bit;

pub fn mask_occupancy(index: u64, bits_in_mask: u8, attack_mask: u64) -> u64 {
    let mut attack_mask = attack_mask;

    let mut occupancy = 0u64;

    for count in 0..bits_in_mask {
        let square = attack_mask.trailing_zeros() as u8;

        clear_bit(&mut attack_mask, square);

        if (index & (1u64 << count)) != 0 {
            occupancy |= 1u64 << square;
        }
    }

    occupancy
}
