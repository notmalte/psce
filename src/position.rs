use std::fmt::Display;

use crate::{
    bitboard::{rf_to_square, square_to_string, string_to_square, BitboardContainer},
    constants::{A1, A8, C1, C8, D1, D8, E1, E8, F1, F8, G1, G8, H1, H8},
    enums::{CastlingRights, Color, PieceAndColor},
    movegen::{Move, MoveGen},
};

#[derive(Clone)]
pub struct Position {
    bitboards: BitboardContainer,
    color_to_move: Color,
    en_passant_square: Option<u8>,
    castling_rights: u8,
}

impl Position {
    pub const INITIAL_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn empty() -> Self {
        Self {
            bitboards: BitboardContainer::empty(),
            color_to_move: Color::White,
            en_passant_square: None,
            castling_rights: CastlingRights::None.flag(),
        }
    }

    pub fn bitboards(&self) -> &BitboardContainer {
        &self.bitboards
    }

    pub fn bitboards_mut(&mut self) -> &mut BitboardContainer {
        &mut self.bitboards
    }

    pub fn color_to_move(&self) -> Color {
        self.color_to_move
    }

    pub fn color_to_move_mut(&mut self) -> &mut Color {
        &mut self.color_to_move
    }

    pub fn en_passant_square(&self) -> Option<u8> {
        self.en_passant_square
    }

    pub fn en_passant_square_mut(&mut self) -> &mut Option<u8> {
        &mut self.en_passant_square
    }

    pub fn castling_rights(&self) -> u8 {
        self.castling_rights
    }

    pub fn castling_rights_mut(&mut self) -> &mut u8 {
        &mut self.castling_rights
    }

    // empty    "8/8/8/8/8/8/8/8 w - - "
    // initial  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    // tricky   "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
    // killer   "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1"
    // cmk      "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9"
    pub fn from_fen(fen: String) -> Result<Self, String> {
        let mut position = Self::empty();

        let fen_parts: Vec<_> = fen.split_whitespace().collect();

        if fen_parts.len() < 4 {
            return Err("FEN string is too short".to_string());
        }

        let ranks: Vec<_> = fen_parts[0].split('/').collect();

        if ranks.len() != 8 {
            return Err("Invalid number of ranks".to_string());
        }

        for (rank, rank_string) in ranks.iter().enumerate() {
            let mut file = 0;

            for c in rank_string.chars() {
                if let Some(n) = c.to_digit(10) {
                    file += n as usize;
                } else {
                    let square = rf_to_square(rank as u8, file as u8);

                    let piece = PieceAndColor::from_char(c)?;

                    position.bitboards.piece_set_bit(piece, square);
                    position.bitboards.color_set_bit(piece.color(), square);

                    file += 1;
                }
            }

            if file != 8 {
                return Err(format!("Invalid number of files in rank {}", 8 - rank));
            }
        }

        position.color_to_move = match fen_parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid side to move".to_string()),
        };

        position.castling_rights = CastlingRights::string_to_flags(fen_parts[2].to_string())?;

        position.en_passant_square = if fen_parts[3] == "-" {
            None
        } else {
            Some(string_to_square(fen_parts[3].to_string())?)
        };

        *position.bitboards.color_mut(Color::Both) =
            position.bitboards.color(Color::White) | position.bitboards.color(Color::Black);

        Ok(position)
    }

    fn make_pseudo_legal_move(&mut self, m: &Move) {
        let color = m.piece.color();

        let opponent_pieces = match color {
            Color::White => PieceAndColor::BLACK_PIECES,
            Color::Black => PieceAndColor::WHITE_PIECES,
            _ => unreachable!(),
        };

        self.bitboards
            .piece_clear_bit_incl_color(m.piece, m.source_square);

        self.bitboards
            .piece_set_bit_incl_color(m.promotion_piece.unwrap_or(m.piece), m.target_square);

        if m.is_capture {
            if m.is_en_passant {
                let (pawn_target_square, pawn_piece) = match color {
                    Color::White => (m.target_square + 8, PieceAndColor::BlackPawn),
                    Color::Black => (m.target_square - 8, PieceAndColor::WhitePawn),
                    _ => unreachable!(),
                };

                self.bitboards
                    .piece_clear_bit_incl_color(pawn_piece, pawn_target_square);
            } else {
                for piece in opponent_pieces {
                    if self.bitboards.piece_get_bit(piece, m.target_square) {
                        self.bitboards
                            .piece_clear_bit_incl_color(piece, m.target_square);
                        break;
                    }
                }
            }
        }

        if m.is_double_pawn_push {
            self.en_passant_square = Some(match color {
                Color::White => m.target_square + 8,
                Color::Black => m.target_square - 8,
                _ => unreachable!(),
            });
        } else {
            self.en_passant_square = None;
        }

        if m.is_castling {
            let (rook_source_square, rook_target_square) = match m.target_square {
                C8 => (A8, D8),
                G8 => (H8, F8),
                C1 => (A1, D1),
                G1 => (H1, F1),
                _ => unreachable!(),
            };

            let rook_piece = match color {
                Color::White => PieceAndColor::WhiteRook,
                Color::Black => PieceAndColor::BlackRook,
                _ => unreachable!(),
            };

            self.bitboards
                .piece_clear_bit_incl_color(rook_piece, rook_source_square);
            self.bitboards
                .piece_set_bit_incl_color(rook_piece, rook_target_square);

            self.castling_rights &= !(match color {
                Color::White => {
                    CastlingRights::WhiteKingside.flag() | CastlingRights::WhiteQueenside.flag()
                }
                Color::Black => {
                    CastlingRights::BlackKingside.flag() | CastlingRights::BlackQueenside.flag()
                }
                _ => unreachable!(),
            });
        } else {
            let castling_move_mask = match m.source_square {
                A8 => CastlingRights::BlackQueenside.flag(),
                E8 => CastlingRights::BlackKingside.flag() | CastlingRights::BlackQueenside.flag(),
                H8 => CastlingRights::BlackKingside.flag(),
                A1 => CastlingRights::WhiteQueenside.flag(),
                E1 => CastlingRights::WhiteKingside.flag() | CastlingRights::WhiteQueenside.flag(),
                H1 => CastlingRights::WhiteKingside.flag(),
                _ => 0,
            };

            self.castling_rights &= !castling_move_mask;

            if m.is_capture {
                let castling_capture_mask = match m.target_square {
                    A8 => CastlingRights::BlackQueenside.flag(),
                    H8 => CastlingRights::BlackKingside.flag(),
                    A1 => CastlingRights::WhiteQueenside.flag(),
                    H1 => CastlingRights::WhiteKingside.flag(),
                    _ => 0,
                };

                self.castling_rights &= !castling_capture_mask;
            }
        }

        self.bitboards.compute_both();

        self.color_to_move = !self.color_to_move;
    }

    pub fn make_move(&self, move_gen: &MoveGen, m: &Move, only_captures: bool) -> Option<Self> {
        if only_captures && !m.is_capture {
            return None;
        }

        let mut clone = self.clone();

        clone.make_pseudo_legal_move(m);

        let king_piece = match self.color_to_move {
            Color::White => PieceAndColor::WhiteKing,
            Color::Black => PieceAndColor::BlackKing,
            _ => unreachable!(),
        };

        let king_square = clone.bitboards.piece(king_piece).trailing_zeros() as u8;

        if move_gen.is_square_attacked(&clone, king_square, !self.color_to_move) {
            return None;
        }

        Some(clone)
    }

    // // init start position
    // position startpos

    // // init start position and make the moves on chess board
    // position startpos moves e2e4 e7e5

    // // init position from FEN string
    // position fen r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1

    // // init position from fen string and make moves on chess board
    // position fen r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 moves e2a6 e8g8
    pub fn from_uci(move_gen: &MoveGen, command: String) -> Option<Self> {
        let command_parts: Vec<_> = command.split_whitespace().collect();

        if command_parts.len() < 2 || command_parts[0] != "position" {
            return None;
        }

        if command_parts[1] == "startpos" {
            if command_parts.len() > 2 && command_parts[2] == "moves" {
                let mut position = Self::default();

                for move_string in command_parts[3..].iter() {
                    let m_opt = move_gen.parse_uci_move(&position, move_string.to_string());

                    if m_opt.is_none() {
                        return None;
                    }

                    position.make_pseudo_legal_move(&m_opt.unwrap());
                }

                return Some(position);
            } else {
                return Some(Self::default());
            }
        } else if command_parts[1] == "fen" {
            if command_parts.len() < 8 {
                return None;
            }

            let fen = command_parts[2..8].join(" ");

            if command_parts.len() > 8 && command_parts[8] == "moves" {
                let position_result = Self::from_fen(fen);

                if position_result.is_err() {
                    return None;
                }

                let mut position = position_result.unwrap();

                for move_string in command_parts[9..].iter() {
                    let m_opt = move_gen.parse_uci_move(&position, move_string.to_string());

                    if m_opt.is_none() {
                        return None;
                    }

                    position.make_pseudo_legal_move(&m_opt.unwrap());
                }

                return Some(position);
            } else {
                let position_result = Self::from_fen(fen);

                if position_result.is_err() {
                    return None;
                }

                return Some(position_result.unwrap());
            }
        }

        None
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.bitboards)?;

        writeln!(f, "Side to move: \t\t{}", self.color_to_move)?;
        writeln!(
            f,
            "En passant square: \t{}",
            if let Some(square) = self.en_passant_square {
                square_to_string(square).unwrap()
            } else {
                "-".to_string()
            }
        )?;
        writeln!(
            f,
            "Castling rights: \t{}",
            CastlingRights::flags_to_string(self.castling_rights)
        )?;

        Ok(())
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::from_fen(Self::INITIAL_FEN.to_string()).unwrap()
    }
}
