use psce_core::{Bitboard, Square};

pub fn generate_knight_attacks() -> [Bitboard; 64] {
    let mut attacks = [Bitboard::empty(); 64];

    for square in Bitboard::all_squares() {
        let bb = Square::to_bb(square);

        let mut mask = Bitboard::empty();

        mask |= bb.north().north().east() & Bitboard::NOT_FILE_A;
        mask |= bb.north().east().east() & Bitboard::NOT_FILE_AB;
        mask |= bb.south().east().east() & Bitboard::NOT_FILE_AB;
        mask |= bb.south().south().east() & Bitboard::NOT_FILE_A;
        mask |= bb.south().south().west() & Bitboard::NOT_FILE_H;
        mask |= bb.south().west().west() & Bitboard::NOT_FILE_GH;
        mask |= bb.north().west().west() & Bitboard::NOT_FILE_GH;
        mask |= bb.north().north().west() & Bitboard::NOT_FILE_H;

        attacks[square as usize] = mask;
    }

    attacks
}
