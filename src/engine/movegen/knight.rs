use crate::engine::{
    bitboard::{Bitboard, Square, NOT_FILE_A, NOT_FILE_AB, NOT_FILE_GH, NOT_FILE_H},
    color::Color,
    moves::{Move, MoveFlags},
    piece::Piece,
    position::Position,
};

pub struct KnightMoveGen {
    attack_table: [Bitboard; 64],
}

impl KnightMoveGen {
    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }

    fn generate_attack_table() -> [Bitboard; 64] {
        let mut table = [Bitboard::empty(); 64];

        for square in Bitboard::all_squares() {
            table[square.to_usize()] = Self::mask_attacks(square);
        }

        table
    }

    fn mask_attacks(square: Square) -> Bitboard {
        let mut attacks = Bitboard::empty();
        let bb = square.to_bb();

        attacks |= (bb >> 17) & NOT_FILE_H;
        attacks |= (bb >> 15) & NOT_FILE_A;
        attacks |= (bb >> 10) & NOT_FILE_GH;
        attacks |= (bb >> 6) & NOT_FILE_AB;
        attacks |= (bb << 6) & NOT_FILE_GH;
        attacks |= (bb << 10) & NOT_FILE_AB;
        attacks |= (bb << 15) & NOT_FILE_H;
        attacks |= (bb << 17) & NOT_FILE_A;

        attacks
    }

    pub(super) fn get_attacks(&self, square: Square) -> Bitboard {
        self.attack_table[square.to_usize()]
    }

    pub fn generate_moves(&self, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => Piece::WhiteKnight,
            Color::Black => Piece::BlackKnight,
        };

        let mut moves = vec![];

        let knights = position.bitboards().piece(piece);

        for from_square in knights.squares() {
            let attacks = self.get_attacks(from_square) & !position.bitboards().color(color);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_attacks() {
        let got = KnightMoveGen::mask_attacks(Square::A2);
        let expected = Square::B4.to_bb() | Square::C3.to_bb() | Square::C1.to_bb();

        assert_eq!(got, expected);

        let got = KnightMoveGen::mask_attacks(Square::D4);
        let expected = Square::B3.to_bb()
            | Square::B5.to_bb()
            | Square::C2.to_bb()
            | Square::C6.to_bb()
            | Square::E2.to_bb()
            | Square::E6.to_bb()
            | Square::F3.to_bb()
            | Square::F5.to_bb();

        assert_eq!(got, expected);
    }

    #[test]
    fn test_generate_attack_table() {
        let table = KnightMoveGen::generate_attack_table();

        let got = table[Square::E3.to_usize()];
        let expected = Square::C2.to_bb()
            | Square::C4.to_bb()
            | Square::D1.to_bb()
            | Square::D5.to_bb()
            | Square::F1.to_bb()
            | Square::F5.to_bb()
            | Square::G2.to_bb()
            | Square::G4.to_bb();

        assert_eq!(got, expected);
    }
}
