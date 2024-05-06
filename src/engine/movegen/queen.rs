use crate::engine::{
    bitboard::{Bitboard, Square},
    color::Color,
    movegen::{bishop::BishopMoveGen, rook::RookMoveGen},
    moves::{Move, MoveFlags},
    piece::Piece,
    position::Position,
};

pub struct QueenMoveGen {
    bishop: BishopMoveGen,
    rook: RookMoveGen,
}

impl QueenMoveGen {
    pub fn new(bishop: BishopMoveGen, rook: RookMoveGen) -> Self {
        Self { bishop, rook }
    }

    fn get_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        self.bishop.get_attacks(square, occupancy) | self.rook.get_attacks(square, occupancy)
    }

    pub fn generate_moves(&self, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => Piece::WhiteQueen,
            Color::Black => Piece::BlackQueen,
        };

        let mut moves = vec![];

        let queens = position.bitboards().piece(piece);

        for from_square in queens.squares() {
            let attacks = self.get_attacks(from_square, position.bitboards().all())
                & !position.bitboards().color(color);

            for to_square in attacks.squares() {
                let capture = position.bitboards().all().get(to_square);

                let flags = if capture {
                    MoveFlags::CAPTURE
                } else {
                    MoveFlags::NONE
                };

                moves.push(Move::new(from_square, to_square, piece, None, flags));
            }
        }

        moves
    }
}
