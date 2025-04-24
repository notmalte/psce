pub fn magic_index(masked_occupancy: u64, magic_number: u64, shift: u8) -> usize {
    (masked_occupancy.wrapping_mul(magic_number) >> shift) as usize
}
