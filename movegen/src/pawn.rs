use core::{Bitboard, Color, Move, MoveFlags, Piece, Position, Square};

use crate::MoveGen;

impl MoveGen {
    pub(crate) fn generate_pawn_attacks() -> [[Bitboard; 64]; 2] {
        let mut attacks = [[Bitboard::empty(); 64]; 2];

        for square in Bitboard::all_squares() {
            let bb = Square::to_bb(square);

            attacks[Color::White as usize][square as usize] =
                ((bb << 7) & Bitboard::NOT_FILE_H) | ((bb << 9) & Bitboard::NOT_FILE_A);
            attacks[Color::Black as usize][square as usize] =
                ((bb >> 7) & Bitboard::NOT_FILE_A) | ((bb >> 9) & Bitboard::NOT_FILE_H);
        }

        attacks
    }

    pub(crate) fn pawn_pseudo_legals(&self, position: &Position, moves: &mut Vec<Move>) {
        let color = position.side_to_move();

        let (promotion_rank, double_push_rank) = match color {
            Color::White => (Bitboard::RANK_8, Bitboard::RANK_4),
            Color::Black => (Bitboard::RANK_1, Bitboard::RANK_5),
        };

        let promotion_pieces = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

        let pawns = position.bitboards().piece(color, Piece::Pawn);
        let all = position.bitboards().all();
        let unoccupied = !all;
        let opponent = position.bitboards().color(!color);

        let single_pushes = unoccupied
            & match color {
                Color::White => pawns << 8,
                Color::Black => pawns >> 8,
            };

        for to_square in single_pushes.squares() {
            let from_square = match color {
                Color::White => to_square - 8,
                Color::Black => to_square + 8,
            };

            if promotion_rank.get(to_square) {
                for promotion_piece in promotion_pieces {
                    moves.push(Move::new(
                        from_square,
                        to_square,
                        Piece::Pawn,
                        Some(promotion_piece),
                        MoveFlags::NONE,
                    ));
                }
            } else {
                moves.push(Move::new(
                    from_square,
                    to_square,
                    Piece::Pawn,
                    None,
                    MoveFlags::NONE,
                ));
            }
        }

        let double_pushes = unoccupied
            & double_push_rank
            & match color {
                Color::White => single_pushes << 8,
                Color::Black => single_pushes >> 8,
            };

        for to_square in double_pushes.squares() {
            let from_square = match color {
                Color::White => to_square - 16,
                Color::Black => to_square + 16,
            };

            moves.push(Move::new(
                from_square,
                to_square,
                Piece::Pawn,
                None,
                MoveFlags::DOUBLE_PUSH,
            ));
        }

        let east_attacks = match color {
            Color::White => pawns << 7,
            Color::Black => pawns >> 9,
        } & Bitboard::NOT_FILE_H;

        let east_captures = east_attacks & opponent;

        for to_square in east_captures.squares() {
            let from_square = match color {
                Color::White => to_square - 7,
                Color::Black => to_square + 9,
            };

            if promotion_rank.get(to_square) {
                for promotion_piece in promotion_pieces {
                    moves.push(Move::new(
                        from_square,
                        to_square,
                        Piece::Pawn,
                        Some(promotion_piece),
                        MoveFlags::CAPTURE,
                    ));
                }
            } else {
                moves.push(Move::new(
                    from_square,
                    to_square,
                    Piece::Pawn,
                    None,
                    MoveFlags::CAPTURE,
                ));
            }
        }

        let west_attacks = match color {
            Color::White => pawns << 9,
            Color::Black => pawns >> 7,
        } & Bitboard::NOT_FILE_A;

        let west_captures = west_attacks & opponent;

        for to_square in west_captures.squares() {
            let from_square = match color {
                Color::White => to_square - 9,
                Color::Black => to_square + 7,
            };

            if promotion_rank.get(to_square) {
                for promotion_piece in promotion_pieces {
                    moves.push(Move::new(
                        from_square,
                        to_square,
                        Piece::Pawn,
                        Some(promotion_piece),
                        MoveFlags::CAPTURE,
                    ));
                }
            } else {
                moves.push(Move::new(
                    from_square,
                    to_square,
                    Piece::Pawn,
                    None,
                    MoveFlags::CAPTURE,
                ));
            }
        }

        if let Some(en_passant_square) = position.en_passant_square() {
            if east_attacks.get(en_passant_square) {
                let from_square = match color {
                    Color::White => en_passant_square - 7,
                    Color::Black => en_passant_square + 9,
                };

                moves.push(Move::new(
                    from_square,
                    en_passant_square,
                    Piece::Pawn,
                    None,
                    MoveFlags::CAPTURE | MoveFlags::EN_PASSANT,
                ));
            }

            if west_attacks.get(en_passant_square) {
                let from_square = match color {
                    Color::White => en_passant_square - 9,
                    Color::Black => en_passant_square + 7,
                };

                moves.push(Move::new(
                    from_square,
                    en_passant_square,
                    Piece::Pawn,
                    None,
                    MoveFlags::CAPTURE | MoveFlags::EN_PASSANT,
                ));
            }
        }
    }
}
