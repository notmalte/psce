use crate::engine::{
    bitboard::{Bitboard, Square, NOT_FILE_A, NOT_FILE_H, RANK_1, RANK_4, RANK_5, RANK_8},
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

    pub(super) fn get_attacks(&self, color: Color, square: Square) -> Bitboard {
        self.attack_table[color.to_usize()][square.to_usize()]
    }

    pub fn generate_pseudo_legal_moves(&self, position: &Position) -> Vec<Move> {
        let color = position.color_to_move();

        let piece = match color {
            Color::White => Piece::WhitePawn,
            Color::Black => Piece::BlackPawn,
        };

        let promotion_rank = match color {
            Color::White => RANK_8,
            Color::Black => RANK_1,
        };

        let double_push_rank = match color {
            Color::White => RANK_4,
            Color::Black => RANK_5,
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
        let all = position.bitboards().all();
        let opponent = position.bitboards().color(!color);

        let single_pushes = !all
            & match color {
                Color::White => pawns >> 8,
                Color::Black => pawns << 8,
            };

        for to_square in single_pushes.squares() {
            let from_square = match color {
                Color::White => to_square + 8,
                Color::Black => to_square - 8,
            };

            if promotion_rank.get(to_square) {
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
            }
        }

        let double_pushes = !all
            & double_push_rank
            & match color {
                Color::White => single_pushes >> 8,
                Color::Black => single_pushes << 8,
            };

        for to_square in double_pushes.squares() {
            let from_square = match color {
                Color::White => to_square + 16,
                Color::Black => to_square - 16,
            };

            moves.push(Move::new(
                from_square,
                to_square,
                piece,
                None,
                MoveFlags::DOUBLE_PUSH,
            ));
        }

        let east_attacks = match color {
            Color::White => pawns >> 9,
            Color::Black => pawns << 7,
        } & NOT_FILE_H;

        let east_captures = east_attacks & opponent;

        for to_square in east_captures.squares() {
            let from_square = match color {
                Color::White => to_square + 9,
                Color::Black => to_square - 7,
            };

            if promotion_rank.get(to_square) {
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

        let west_attacks = match color {
            Color::White => pawns >> 7,
            Color::Black => pawns << 9,
        } & NOT_FILE_A;

        let west_captures = west_attacks & opponent;

        for to_square in west_captures.squares() {
            let from_square = match color {
                Color::White => to_square + 7,
                Color::Black => to_square - 9,
            };

            if promotion_rank.get(to_square) {
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
            if east_attacks.get(en_passant_square) {
                let from_square = match color {
                    Color::White => en_passant_square + 9,
                    Color::Black => en_passant_square - 7,
                };

                moves.push(Move::new(
                    from_square,
                    en_passant_square,
                    piece,
                    None,
                    MoveFlags::CAPTURE | MoveFlags::EN_PASSANT,
                ));
            }

            if west_attacks.get(en_passant_square) {
                let from_square = match color {
                    Color::White => en_passant_square + 7,
                    Color::Black => en_passant_square - 9,
                };

                moves.push(Move::new(
                    from_square,
                    en_passant_square,
                    piece,
                    None,
                    MoveFlags::CAPTURE | MoveFlags::EN_PASSANT,
                ));
            }
        }

        moves
    }
}

impl Default for PawnMoveGen {
    fn default() -> Self {
        Self::new()
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
