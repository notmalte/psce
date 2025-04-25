use core::Bitboard;

pub fn magic_index(masked_occupancy: Bitboard, magic_number: u64, shift: u8) -> usize {
    (masked_occupancy.to_repr().wrapping_mul(magic_number) >> shift) as usize
}
