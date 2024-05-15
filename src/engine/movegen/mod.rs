use crate::engine::{
    bitboard::Square,
    color::Color,
    movegen::{
        bishop::BishopMoveGen, king::KingMoveGen, knight::KnightMoveGen, pawn::PawnMoveGen,
        queen::QueenMoveGen, rook::RookMoveGen,
    },
    moves::Move,
    piece::Piece,
    position::Position,
};

mod bishop;
mod king;
mod knight;
mod magic;
mod occupancy;
mod pawn;
mod queen;
mod rook;

pub struct MoveGen {
    pawn: PawnMoveGen,
    knight: KnightMoveGen,
    bishop: BishopMoveGen,
    rook: RookMoveGen,
    queen: QueenMoveGen,
    king: KingMoveGen,
}

impl MoveGen {
    pub fn new() -> Self {
        let pawn = PawnMoveGen::new();
        let knight = KnightMoveGen::new();
        let bishop = BishopMoveGen::new();
        let rook = RookMoveGen::new();
        let queen = QueenMoveGen::new(bishop.clone(), rook.clone());
        let king = KingMoveGen::new();

        Self {
            pawn,
            knight,
            bishop,
            rook,
            queen,
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

    pub fn rook(&self) -> &RookMoveGen {
        &self.rook
    }

    pub fn queen(&self) -> &QueenMoveGen {
        &self.queen
    }

    pub fn king(&self) -> &KingMoveGen {
        &self.king
    }

    pub fn is_attacked(&self, position: &Position, square: Square, attacker_color: Color) -> bool {
        let (pawn, knight, bishop, rook, queen, king) = match attacker_color {
            Color::White => (
                Piece::WhitePawn,
                Piece::WhiteKnight,
                Piece::WhiteBishop,
                Piece::WhiteRook,
                Piece::WhiteQueen,
                Piece::WhiteKing,
            ),
            Color::Black => (
                Piece::BlackPawn,
                Piece::BlackKnight,
                Piece::BlackBishop,
                Piece::BlackRook,
                Piece::BlackQueen,
                Piece::BlackKing,
            ),
        };

        if (self.pawn.get_attacks(!attacker_color, square) & position.bitboards().piece(pawn))
            .is_not_empty()
        {
            return true;
        }

        if (self.knight.get_attacks(square) & position.bitboards().piece(knight)).is_not_empty() {
            return true;
        }

        if (self.bishop.get_attacks(square, position.bitboards().all())
            & (position.bitboards().piece(bishop)))
        .is_not_empty()
        {
            return true;
        }

        if (self.rook.get_attacks(square, position.bitboards().all())
            & (position.bitboards().piece(rook)))
        .is_not_empty()
        {
            return true;
        }

        if (self.queen.get_attacks(square, position.bitboards().all())
            & (position.bitboards().piece(queen)))
        .is_not_empty()
        {
            return true;
        }

        if (self.king.get_attacks(square) & position.bitboards().piece(king)).is_not_empty() {
            return true;
        }

        false
    }

    pub fn generate_pseudo_legal_moves(&self, position: &Position) -> Vec<Move> {
        let mut moves = vec![];

        moves.extend(self.pawn.generate_pseudo_legal_moves(position));
        moves.extend(self.knight.generate_pseudo_legal_moves(position));
        moves.extend(self.bishop.generate_pseudo_legal_moves(position));
        moves.extend(self.rook.generate_pseudo_legal_moves(position));
        moves.extend(self.queen.generate_pseudo_legal_moves(position));
        moves.extend(self.king.generate_pseudo_legal_moves(position, self));

        moves
    }

    pub fn generate_legal_moves(&self, position: &Position) -> Vec<(Move, Position)> {
        let mut moves = vec![];

        for pseudo_move in self.generate_pseudo_legal_moves(position) {
            if let Some(new_position) = position.make_move(self, &pseudo_move) {
                moves.push((pseudo_move, new_position));
            }
        }

        moves
    }
}
