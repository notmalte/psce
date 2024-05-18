use std::{fmt::Display, str::FromStr};

use crate::engine::{
    bitboard::{BitboardContainer, Square},
    castling::Castling,
    color::Color,
    movegen::MoveGen,
    moves::Move,
    piece::Piece,
};

pub const FEN_INITIAL_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN_TEST_POSITION_1: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
pub const FEN_TEST_POSITION_2: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
pub const FEN_TEST_POSITION_3: &str =
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - ";
pub const FEN_TEST_POSITION_4: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
pub const FEN_TEST_POSITION_5: &str =
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - -";

#[derive(Clone)]
pub struct Position {
    bitboards: BitboardContainer,
    color_to_move: Color,
    castling: Castling,
    en_passant_square: Option<Square>,
}

impl Position {
    pub fn empty() -> Self {
        Self {
            bitboards: BitboardContainer::empty(),
            color_to_move: Color::White,
            castling: Castling::NONE,
            en_passant_square: None,
        }
    }

    pub fn initial() -> Self {
        Self::from_fen(FEN_INITIAL_POSITION).unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut position = Self::empty();

        let parts: Vec<_> = fen.split_whitespace().collect();

        if parts.len() < 4 {
            return Err("FEN string too short".to_string());
        }

        let rows: Vec<_> = parts[0].split('/').collect();

        if rows.len() != 8 {
            return Err("Invalid number of rows".to_string());
        }

        for (y, row) in rows.iter().enumerate() {
            let mut x = 0;

            for c in row.chars() {
                if x >= 8 {
                    return Err(format!("Invalid length of row {}", y));
                }

                if let Some(n) = c.to_digit(10) {
                    x += n as usize;
                } else {
                    let piece = Piece::from_char(c).ok_or_else(|| "Invalid piece".to_string())?;

                    let square = Square::from_xy(x as u8, y as u8).unwrap();

                    position.bitboards.piece_mut(piece).set(square);
                    position.bitboards.color_mut(piece.color()).set(square);

                    x += 1;
                }
            }

            if x != 8 {
                return Err(format!("Invalid length of row {}", y));
            }
        }

        position.color_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid side to move".to_string()),
        };

        position.castling = Castling::from_str(parts[2])?;

        position.en_passant_square = match parts[3] {
            "-" => None,
            s => Some(Square::from_str(s)?),
        };

        *position.bitboards.all_mut() =
            position.bitboards.color(Color::White) | position.bitboards.color(Color::Black);

        Ok(position)
    }

    pub fn bitboards(&self) -> &BitboardContainer {
        &self.bitboards
    }

    pub fn color_to_move(&self) -> Color {
        self.color_to_move
    }

    pub fn castling(&self) -> Castling {
        self.castling
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    pub fn make_move(&self, mg: &MoveGen, m: &Move) -> Option<Self> {
        let mut clone = self.clone();

        clone.apply_move(m);

        let king = match self.color_to_move {
            Color::White => Piece::WhiteKing,
            Color::Black => Piece::BlackKing,
        };

        let king_square = clone.bitboards.piece(king).last_square().unwrap();

        if mg.is_attacked(&clone, king_square, clone.color_to_move) {
            return None;
        }

        Some(clone)
    }

    fn apply_move(&mut self, m: &Move) {
        let opponent = !self.color_to_move;

        self.bitboards.piece_mut(m.piece()).clear(m.from());
        self.bitboards.color_mut(self.color_to_move).clear(m.from());

        if let Some(promoted) = m.promotion() {
            self.bitboards.piece_mut(promoted).set(m.to());
        } else {
            self.bitboards.piece_mut(m.piece()).set(m.to());
        }
        self.bitboards.color_mut(self.color_to_move).set(m.to());

        if m.flags().is_capture() {
            if m.flags().is_en_passant() {
                let (square, piece) = match self.color_to_move {
                    Color::White => (m.to() + 8, Piece::BlackPawn),
                    Color::Black => (m.to() - 8, Piece::WhitePawn),
                };

                self.bitboards.piece_mut(piece).clear(square);
                self.bitboards.color_mut(opponent).clear(square);
            } else {
                let pieces = match self.color_to_move {
                    Color::White => Piece::BLACK_PIECES,
                    Color::Black => Piece::WHITE_PIECES,
                };

                for piece in pieces {
                    self.bitboards.piece_mut(piece).clear(m.to());
                }
                self.bitboards.color_mut(opponent).clear(m.to());

                let castling_mask = match m.to() {
                    Square::A8 => Castling::BLACK_QUEEN_SIDE,
                    Square::H8 => Castling::BLACK_KING_SIDE,
                    Square::A1 => Castling::WHITE_QUEEN_SIDE,
                    Square::H1 => Castling::WHITE_KING_SIDE,
                    _ => Castling::NONE,
                };

                self.castling.clear(castling_mask);
            }
        }

        if m.flags().is_double_push() {
            self.en_passant_square = match self.color_to_move {
                Color::White => Some(m.to() + 8),
                Color::Black => Some(m.to() - 8),
            };
        } else {
            self.en_passant_square = None;
        }

        if m.flags().is_castling() {
            let (rook_from, rook_to) = match m.to() {
                Square::C8 => (Square::A8, Square::D8),
                Square::G8 => (Square::H8, Square::F8),
                Square::C1 => (Square::A1, Square::D1),
                Square::G1 => (Square::H1, Square::F1),
                _ => unreachable!(),
            };

            let rook = match self.color_to_move {
                Color::White => Piece::WhiteRook,
                Color::Black => Piece::BlackRook,
            };

            self.bitboards.piece_mut(rook).clear(rook_from);
            self.bitboards
                .color_mut(self.color_to_move)
                .clear(rook_from);

            self.bitboards.piece_mut(rook).set(rook_to);
            self.bitboards.color_mut(self.color_to_move).set(rook_to);

            let castling_mask = match self.color_to_move {
                Color::White => Castling::WHITE_ALL,
                Color::Black => Castling::BLACK_ALL,
            };

            self.castling.clear(castling_mask);
        } else {
            let castling_mask = match m.from() {
                Square::A8 => Castling::BLACK_QUEEN_SIDE,
                Square::E8 => Castling::BLACK_ALL,
                Square::H8 => Castling::BLACK_KING_SIDE,
                Square::A1 => Castling::WHITE_QUEEN_SIDE,
                Square::E1 => Castling::WHITE_ALL,
                Square::H1 => Castling::WHITE_KING_SIDE,
                _ => Castling::NONE,
            };

            self.castling.clear(castling_mask);
        }

        *self.bitboards.all_mut() =
            self.bitboards.color(Color::White) | self.bitboards.color(Color::Black);

        self.color_to_move = opponent;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n", self.bitboards)?;

        writeln!(f, "Side to move: {}", self.color_to_move)?;
        writeln!(f, "Castling rights: {}", self.castling)?;
        write!(
            f,
            "En passant square: {}",
            self.en_passant_square
                .map_or("-".to_string(), |s| s.to_string())
        )?;

        Ok(())
    }
}
