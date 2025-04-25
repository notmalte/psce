use core::{Bitboard, Color, Move, Position};

mod bishop;
mod king;
mod knight;
mod magic;
mod pawn;
mod queen;
mod rook;

pub struct MoveGen;

impl MoveGen {
    pub fn pseudo_legals(position: &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        Self::pawn_pseudo_legals(position, &mut moves);
        Self::knight_pseudo_legals(position, &mut moves);
        Self::bishop_pseudo_legals(position, &mut moves);
        Self::rook_pseudo_legals(position, &mut moves);
        Self::queen_pseudo_legals(position, &mut moves);
        Self::king_pseudo_legals(position, &mut moves);

        moves
    }

    pub fn is_attacked(position: &Position, square: u8, by_side: Color) -> bool {
        // TODO

        false
    }
}
