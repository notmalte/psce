use crate::engine::bitboard::Bitboard;

pub const ARRAY_SIZE_BISHOP: usize = 512;
pub const ARRAY_SIZE_ROOK: usize = 4096;
pub const MAX_ATTEMPTS: usize = 1_000_000_000;

pub fn calculate_magic_index(occupancy: Bitboard, magic_number: u64, bits_in_mask: u8) -> usize {
    (occupancy.to_repr().wrapping_mul(magic_number) >> (64 - bits_in_mask)) as usize
}

pub fn generate_magic_number_candidate() -> u64 {
    let mut rng = fastrand::Rng::new();

    let n1 = rng.u64(..);
    let n2 = rng.u64(..);
    let n3 = rng.u64(..);

    n1 & n2 & n3
}
