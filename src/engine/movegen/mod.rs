use crate::engine::{
    bitboard::Square,
    color::Color,
    movegen::{king::KingMoveGen, knight::KnightMoveGen, pawn::PawnMoveGen},
    position::Position,
};

mod king;
mod knight;
mod pawn;

pub struct MoveGen {
    pawn: PawnMoveGen,
    knight: KnightMoveGen,
    king: KingMoveGen,
    // rook: !,
    // bishop: !,
    // queen: !,
}

impl MoveGen {
    pub fn new() -> Self {
        let pawn = PawnMoveGen::new();
        let knight = KnightMoveGen::new();
        let king = KingMoveGen::new();

        Self { pawn, knight, king }
    }

    pub fn pawn(&self) -> &PawnMoveGen {
        &self.pawn
    }

    pub fn knight(&self) -> &KnightMoveGen {
        &self.knight
    }

    pub fn king(&self) -> &KingMoveGen {
        &self.king
    }

    fn is_attacked(&self, position: &Position, square: Square, attacker_color: Color) -> bool {
        // TODO
        false
    }
}
