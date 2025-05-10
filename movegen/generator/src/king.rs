use psce_core::{Bitboard, Square};

pub fn generate_king_attacks() -> [Bitboard; 64] {
    let mut attacks = [Bitboard::empty(); 64];

    for square in Bitboard::all_squares() {
        let bb = Square::to_bb(square);

        let mut mask = Bitboard::empty();

        mask |= bb.north();
        mask |= bb.north().east() & Bitboard::NOT_FILE_A;
        mask |= bb.east() & Bitboard::NOT_FILE_A;
        mask |= bb.south().east() & Bitboard::NOT_FILE_A;
        mask |= bb.south();
        mask |= bb.south().west() & Bitboard::NOT_FILE_H;
        mask |= bb.west() & Bitboard::NOT_FILE_H;
        mask |= bb.north().west() & Bitboard::NOT_FILE_H;

        attacks[square as usize] = mask;
    }

    attacks
}
