use core::{Color, Move, Piece, Position};

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

    pub fn legals(position: &Position) -> Vec<Move> {
        Self::pseudo_legals(position)
            .into_iter()
            .filter(|m| {
                let mut p = position.clone();
                let own_color = p.side_to_move();

                p.make_move(m);

                let king_square = p.king_square(own_color).expect("should have a king");
                let is_king_attacked = Self::is_attacked(&p, king_square, p.side_to_move());

                !is_king_attacked
            })
            .collect()
    }

    pub fn is_attacked(position: &Position, square: u8, by_side: Color) -> bool {
        if (Self::pawn_attacks(!by_side, square) & position.bitboards().piece(by_side, Piece::Pawn))
            .is_not_empty()
        {
            return true;
        }

        if (Self::knight_attacks(square) & position.bitboards().piece(by_side, Piece::Knight))
            .is_not_empty()
        {
            return true;
        }

        let all = position.bitboards().all();

        if (Self::bishop_attacks(square, all) & position.bitboards().piece(by_side, Piece::Bishop))
            .is_not_empty()
        {
            return true;
        }

        if (Self::rook_attacks(square, all) & position.bitboards().piece(by_side, Piece::Rook))
            .is_not_empty()
        {
            return true;
        }

        // potential optimization: test in bishop and rook
        if (Self::queen_attacks(square, all) & position.bitboards().piece(by_side, Piece::Queen))
            .is_not_empty()
        {
            return true;
        }

        if (Self::king_attacks(square) & position.bitboards().piece(by_side, Piece::King))
            .is_not_empty()
        {
            return true;
        }

        false
    }
}
