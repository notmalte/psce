use psce_core::{Bitboard, Color, Square};

pub fn generate_pawn_attacks() -> [Bitboard; 128] {
    let mut attacks = [Bitboard::empty(); 128];

    for square in Bitboard::all_squares() {
        let bb = Square::to_bb(square);

        attacks[square as usize + (Color::White as usize * 64)] =
            (bb.north().east() & Bitboard::NOT_FILE_A) | (bb.north().west() & Bitboard::NOT_FILE_H);
        attacks[square as usize + (Color::Black as usize * 64)] =
            (bb.south().east() & Bitboard::NOT_FILE_A) | (bb.south().west() & Bitboard::NOT_FILE_H);
    }

    attacks
}
