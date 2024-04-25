use crate::engine::movegen::{knight::KnightMoveGen, pawn::PawnMoveGen};

mod knight;
mod pawn;

pub struct MoveGen {
    pawn: PawnMoveGen,
    knight: KnightMoveGen,
    // king: !,
    // rook: !,
    // bishop: !,
    // queen: !,
}

impl MoveGen {
    pub fn new() -> Self {
        let pawn = PawnMoveGen::new();
        let knight = KnightMoveGen::new();

        Self { pawn, knight }
    }

    pub fn pawn(&self) -> &PawnMoveGen {
        &self.pawn
    }

    pub fn knight(&self) -> &KnightMoveGen {
        &self.knight
    }
}
