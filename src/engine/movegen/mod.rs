use crate::engine::{
    bitboard::Square,
    color::Color,
    movegen::{bishop::BishopMoveGen, king::KingMoveGen, knight::KnightMoveGen, pawn::PawnMoveGen},
    position::Position,
};

mod bishop;
mod king;
mod knight;
mod pawn;

pub struct MoveGen {
    pawn: PawnMoveGen,
    knight: KnightMoveGen,
    bishop: BishopMoveGen,
    // rook: !,
    // queen: !,
    king: KingMoveGen,
}

impl MoveGen {
    pub fn new() -> Self {
        let pawn = PawnMoveGen::new();
        let knight = KnightMoveGen::new();
        let bishop = BishopMoveGen::new();
        // let rook = RookMoveGen::new();
        // let queen = QueenMoveGen::new();
        let king = KingMoveGen::new();

        Self {
            pawn,
            knight,
            bishop,
            king,
        }
    }

    pub fn pawn(&self) -> &PawnMoveGen {
        &self.pawn
    }

    pub fn knight(&self) -> &KnightMoveGen {
        &self.knight
    }

    pub fn bishop(&self) -> &BishopMoveGen {
        &self.bishop
    }

    pub fn king(&self) -> &KingMoveGen {
        &self.king
    }

    fn is_attacked(&self, position: &Position, square: Square, attacker_color: Color) -> bool {
        // TODO
        false
    }
}
