mod bishop;
mod king;
mod knight;
mod occupancy;
mod pawn;
mod queen;
mod rook;

pub use bishop::*;
pub use king::*;
pub use knight::*;
pub use occupancy::*;
pub use pawn::*;
pub use queen::*;
pub use rook::*;

pub struct MoveGen {
    pawn: PawnMoveGen,
    knight: KnightMoveGen,
    king: KingMoveGen,
    rook: RookMoveGen,
    bishop: BishopMoveGen,
    queen: QueenMoveGen,
}

impl MoveGen {
    pub fn pawn(&self) -> &PawnMoveGen {
        &self.pawn
    }

    pub fn knight(&self) -> &KnightMoveGen {
        &self.knight
    }

    pub fn king(&self) -> &KingMoveGen {
        &self.king
    }

    pub fn rook(&self) -> &RookMoveGen {
        &self.rook
    }

    pub fn bishop(&self) -> &BishopMoveGen {
        &self.bishop
    }

    pub fn queen(&self) -> &QueenMoveGen {
        &self.queen
    }

    pub fn new() -> Self {
        let rook = RookMoveGen::new();
        let bishop = BishopMoveGen::new();

        Self {
            pawn: PawnMoveGen::new(),
            knight: KnightMoveGen::new(),
            king: KingMoveGen::new(),
            rook: rook.clone(),
            bishop: bishop.clone(),
            queen: QueenMoveGen::new(rook, bishop),
        }
    }
}
