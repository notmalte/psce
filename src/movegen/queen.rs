use super::{BishopMoveGen, RookMoveGen};

pub struct QueenMoveGen {
    rook: RookMoveGen,
    bishop: BishopMoveGen,
}

impl QueenMoveGen {
    pub fn new(rook: RookMoveGen, bishop: BishopMoveGen) -> Self {
        Self { rook, bishop }
    }

    pub fn get_attacks(&self, square: u8, occupancy: u64) -> u64 {
        self.rook.get_attacks(square, occupancy) | self.bishop.get_attacks(square, occupancy)
    }
}
