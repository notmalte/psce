use crate::engine::{
    bitboard::{Bitboard, Square, NOT_FILE_A, NOT_FILE_H},
    color::Color,
    moves::{Move, MoveFlags},
    piece::Piece,
    position::Position,
};

pub struct PawnMoveGen {
    attack_table: [[Bitboard; 64]; 2],
}

impl PawnMoveGen {
    pub fn new() -> Self {
        Self {
            attack_table: Self::generate_attack_table(),
        }
    }

    fn generate_attack_table() -> [[Bitboard; 64]; 2] {
        let mut table = [[Bitboard::empty(); 64]; 2];

        for square in Bitboard::all_squares() {
            table[Color::White.to_usize()][square.to_usize()] =
                Self::mask_attacks(Color::White, square);
            table[Color::Black.to_usize()][square.to_usize()] =
                Self::mask_attacks(Color::Black, square);
        }

        table
    }

    fn mask_attacks(color: Color, square: Square) -> Bitboard {
        let mut attacks = Bitboard::empty();
        let bb = square.to_bb();

        if color == Color::White {
            attacks |= (bb >> 7) & NOT_FILE_A;
            attacks |= (bb >> 9) & NOT_FILE_H;
        } else {
            attacks |= (bb << 7) & NOT_FILE_H;
            attacks |= (bb << 9) & NOT_FILE_A;
        }

        attacks
    }

    pub fn generate_moves(&self, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => Piece::WhitePawn,
            Color::Black => Piece::BlackPawn,
        };

        let promotion_range = match color {
            Color::White => Square::A8..=Square::H8,
            Color::Black => Square::A1..=Square::H1,
        };

        let home_range = match color {
            Color::White => Square::A2..=Square::H2,
            Color::Black => Square::A7..=Square::H7,
        };

        let promotion_pieces = match color {
            Color::White => [
                Piece::WhiteQueen,
                Piece::WhiteRook,
                Piece::WhiteBishop,
                Piece::WhiteKnight,
            ],
            Color::Black => [
                Piece::BlackQueen,
                Piece::BlackRook,
                Piece::BlackBishop,
                Piece::BlackKnight,
            ],
        };

        let mut moves = vec![];

        let pawns = position.bitboards().piece(piece);

        for from_square in pawns.squares() {
            if promotion_range.contains(&from_square) {
                continue;
            }

            let to_square = match color {
                Color::White => from_square - 8,
                Color::Black => from_square + 8,
            };

            if !position.bitboards().all().get(to_square) {
                if promotion_range.contains(&to_square) {
                    for promotion_piece in promotion_pieces {
                        moves.push(Move::new(
                            from_square,
                            to_square,
                            piece,
                            Some(promotion_piece),
                            MoveFlags::NONE,
                        ));
                    }
                } else {
                    moves.push(Move::new(
                        from_square,
                        to_square,
                        piece,
                        None,
                        MoveFlags::NONE,
                    ));

                    if home_range.contains(&from_square) {
                        let double_push_square = match color {
                            Color::White => from_square - 16,
                            Color::Black => from_square + 16,
                        };

                        if !position.bitboards().all().get(double_push_square) {
                            moves.push(Move::new(
                                from_square,
                                double_push_square,
                                piece,
                                None,
                                MoveFlags::DOUBLE_PUSH,
                            ));
                        }
                    }
                }
            }

            let attacks = self.attack_table[color.to_usize()][from_square.to_usize()];
            let captures = attacks & position.bitboards().color(!color);

            for to_square in captures.squares() {
                if promotion_range.contains(&to_square) {
                    for promotion_piece in promotion_pieces {
                        moves.push(Move::new(
                            from_square,
                            to_square,
                            piece,
                            Some(promotion_piece),
                            MoveFlags::CAPTURE,
                        ));
                    }
                } else {
                    moves.push(Move::new(
                        from_square,
                        to_square,
                        piece,
                        None,
                        MoveFlags::CAPTURE,
                    ));
                }
            }

            if let Some(en_passant_square) = position.en_passant_square() {
                if attacks.get(en_passant_square) {
                    moves.push(Move::new(
                        from_square,
                        en_passant_square,
                        piece,
                        None,
                        MoveFlags::CAPTURE | MoveFlags::EN_PASSANT,
                    ));
                }
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
        let got = PawnMoveGen::mask_attacks(Color::White, Square::A2);
        let expected = Square::B3.to_bb();

        assert_eq!(got, expected);

        let got = PawnMoveGen::mask_attacks(Color::Black, Square::A7);
        let expected = Square::B6.to_bb();

        assert_eq!(got, expected);

        let got = PawnMoveGen::mask_attacks(Color::White, Square::B4);
        let expected = Square::A5.to_bb() | Square::C5.to_bb();

        assert_eq!(got, expected);
    }

    #[test]
    fn test_generate_attack_table() {
        let table = PawnMoveGen::generate_attack_table();

        let got = table[Color::White.to_usize()][Square::E3.to_usize()];
        let expected = Square::D4.to_bb() | Square::F4.to_bb();

        assert_eq!(got, expected);
    }
}
