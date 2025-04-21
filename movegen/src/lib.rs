use core::{Bitboard, Color, Move, Position};

mod king;
mod pawn;

pub struct MoveGen {
    pawn_attacks: [[Bitboard; 64]; 2],
    // ...
    king_attacks: [Bitboard; 64],
}

impl MoveGen {
    pub fn new() -> Self {
        Self {
            pawn_attacks: Self::generate_pawn_attacks(),
            // ...
            king_attacks: Self::generate_king_attacks(),
        }
    }

    pub fn pseudo_legals(&self, position: &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        self.pawn_pseudo_legals(position, &mut moves);
        // self.generate_knight_moves(position, &mut moves);
        // self.generate_bishop_moves(position, &mut moves);
        // self.generate_rook_moves(position, &mut moves);
        // self.generate_queen_moves(position, &mut moves);
        self.king_pseudo_legals(position, &mut moves);

        moves
    }

    pub fn is_attacked(&self, position: &Position, square: u8, by_side: Color) -> bool {
        // TODO

        false
    }
}
