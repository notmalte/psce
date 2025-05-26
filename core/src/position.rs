use std::fmt::Display;

use crate::{
    bitboard::BitboardContainer, castling::Castling, color::Color, moves::Move, piece::Piece,
    square::Square, undo::Undo,
};

pub const FEN_INITIAL_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone)]
pub struct Position {
    bitboards: BitboardContainer,
    side_to_move: Color,
    castling: Castling,
    en_passant_square: Option<u8>,
    halfmove_clock: u8,
    fullmove_number: u16,
    // TODO: hash
}

impl Position {
    pub fn empty() -> Self {
        Self {
            bitboards: BitboardContainer::empty(),
            side_to_move: Color::White,
            castling: Castling::NONE,
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
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
                    let color = if c.is_lowercase() {
                        Color::Black
                    } else {
                        Color::White
                    };

                    let piece = Piece::from_char(c).ok_or("Invalid piece".to_string())?;

                    let square = Square::from_rf(7 - (y as u8), x as u8);

                    position.bitboards.piece_mut(color, piece).set(square);
                    position.bitboards.color_mut(color).set(square);

                    x += 1;
                }
            }

            if x != 8 {
                return Err(format!("Invalid length of row {}", y));
            }
        }

        *position.bitboards.all_mut() =
            position.bitboards.color(Color::White) | position.bitboards.color(Color::Black);

        position.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid side to move".to_string()),
        };

        position.castling = parts[2].parse()?;

        position.en_passant_square = match parts[3] {
            "-" => None,
            s => Some(Square::from_str(s).ok_or("Invalid en passant square".to_string())?),
        };

        position.halfmove_clock = if parts.len() > 4 {
            parts[4]
                .parse()
                .map_err(|_| "Invalid halfmove clock".to_string())?
        } else {
            0
        };

        position.fullmove_number = if parts.len() > 5 {
            parts[5]
                .parse()
                .map_err(|_| "Invalid fullmove number".to_string())?
        } else {
            1
        };

        Ok(position)
    }

    pub fn bitboards(&self) -> &BitboardContainer {
        &self.bitboards
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn castling(&self) -> Castling {
        self.castling
    }

    pub fn en_passant_square(&self) -> Option<u8> {
        self.en_passant_square
    }

    pub fn halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    pub fn fullmove_number(&self) -> u16 {
        self.fullmove_number
    }

    pub fn make_move(&mut self, mv: &Move) -> Undo {
        let mut undo = Undo {
            captured_piece: None,
            previous_castling: self.castling,
            previous_en_passant_square: self.en_passant_square,
            previous_halfmove_clock: self.halfmove_clock,
        };

        let color = self.side_to_move;
        let opponent = !color;

        self.bitboards.piece_mut(color, mv.piece()).clear(mv.from());
        self.bitboards.color_mut(color).clear(mv.from());

        if let Some(promoted) = mv.promotion() {
            self.bitboards.piece_mut(color, promoted).set(mv.to());
        } else {
            self.bitboards.piece_mut(color, mv.piece()).set(mv.to());
        }
        self.bitboards.color_mut(color).set(mv.to());

        if mv.flags().is_capture() {
            if mv.flags().is_en_passant() {
                let square = match color {
                    Color::White => mv.to() - 8,
                    Color::Black => mv.to() + 8,
                };

                undo.captured_piece = Some(Piece::Pawn);
                self.bitboards
                    .piece_mut(opponent, Piece::Pawn)
                    .clear(square);
                self.bitboards.color_mut(opponent).clear(square);
            } else {
                for piece in Piece::ALL {
                    if self.bitboards.piece(opponent, piece).get(mv.to()) {
                        undo.captured_piece = Some(piece);
                        self.bitboards.piece_mut(opponent, piece).clear(mv.to());
                        break;
                    }
                }
                self.bitboards.color_mut(opponent).clear(mv.to());

                let castling_mask = match mv.to() {
                    Square::A1 => Castling::WHITE_QUEEN_SIDE,
                    Square::H1 => Castling::WHITE_KING_SIDE,
                    Square::A8 => Castling::BLACK_QUEEN_SIDE,
                    Square::H8 => Castling::BLACK_KING_SIDE,
                    _ => Castling::NONE,
                };

                self.castling.clear(castling_mask);
            }
        }

        if mv.flags().is_double_push() {
            self.en_passant_square = match color {
                Color::White => Some(mv.to() - 8),
                Color::Black => Some(mv.to() + 8),
            };
        } else {
            self.en_passant_square = None;
        }

        if mv.flags().is_castling() {
            let (rook_from, rook_to) = match mv.to() {
                Square::C1 => (Square::A1, Square::D1),
                Square::G1 => (Square::H1, Square::F1),
                Square::C8 => (Square::A8, Square::D8),
                Square::G8 => (Square::H8, Square::F8),
                _ => unreachable!(),
            };

            self.bitboards
                .piece_mut(color, Piece::Rook)
                .clear(rook_from);
            self.bitboards.color_mut(color).clear(rook_from);

            self.bitboards.piece_mut(color, Piece::Rook).set(rook_to);
            self.bitboards.color_mut(color).set(rook_to);

            let castling_mask = match color {
                Color::White => Castling::WHITE_ALL,
                Color::Black => Castling::BLACK_ALL,
            };

            self.castling.clear(castling_mask);
        } else {
            let castling_mask = match mv.from() {
                Square::A1 => Castling::WHITE_QUEEN_SIDE,
                Square::E1 => Castling::WHITE_ALL,
                Square::H1 => Castling::WHITE_KING_SIDE,
                Square::A8 => Castling::BLACK_QUEEN_SIDE,
                Square::E8 => Castling::BLACK_ALL,
                Square::H8 => Castling::BLACK_KING_SIDE,
                _ => Castling::NONE,
            };

            self.castling.clear(castling_mask);
        }

        *self.bitboards.all_mut() =
            self.bitboards.color(Color::White) | self.bitboards.color(Color::Black);

        if mv.flags().is_capture() || mv.piece() == Piece::Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        if color == Color::Black {
            self.fullmove_number += 1;
        }

        self.side_to_move = opponent;

        undo
    }

    pub fn undo_move(&mut self, mv: &Move, undo: &Undo) {
        self.side_to_move = !self.side_to_move;
        let color = self.side_to_move;
        let opponent = !color;

        self.castling = undo.previous_castling;
        self.en_passant_square = undo.previous_en_passant_square;
        self.halfmove_clock = undo.previous_halfmove_clock;

        if self.side_to_move == Color::Black {
            self.fullmove_number -= 1;
        }

        let current_piece_on_to_sq = mv.promotion().unwrap_or(mv.piece());
        self.bitboards
            .piece_mut(color, current_piece_on_to_sq)
            .clear(mv.to());
        self.bitboards.color_mut(color).clear(mv.to());

        self.bitboards.piece_mut(color, mv.piece()).set(mv.from());
        self.bitboards.color_mut(color).set(mv.from());

        if mv.flags().is_castling() {
            let (rook_from, rook_to) = match mv.to() {
                Square::C1 => (Square::A1, Square::D1),
                Square::G1 => (Square::H1, Square::F1),
                Square::C8 => (Square::A8, Square::D8),
                Square::G8 => (Square::H8, Square::F8),
                _ => unreachable!(),
            };

            self.bitboards.piece_mut(color, Piece::Rook).clear(rook_to);
            self.bitboards.color_mut(color).clear(rook_to);

            self.bitboards.piece_mut(color, Piece::Rook).set(rook_from);
            self.bitboards.color_mut(color).set(rook_from);
        }

        if mv.flags().is_capture() {
            if mv.flags().is_en_passant() {
                let square = match color {
                    Color::White => mv.to() - 8,
                    Color::Black => mv.to() + 8,
                };

                self.bitboards.piece_mut(opponent, Piece::Pawn).set(square);
                self.bitboards.color_mut(opponent).set(square);
            } else {
                let captured_piece = undo.captured_piece.unwrap();

                self.bitboards
                    .piece_mut(opponent, captured_piece)
                    .set(mv.to());
                self.bitboards.color_mut(opponent).set(mv.to());
            }
        }

        *self.bitboards.all_mut() =
            self.bitboards.color(Color::White) | self.bitboards.color(Color::Black);
    }

    pub fn king_square(&self, color: Color) -> Option<u8> {
        self.bitboards.piece(color, Piece::King).last_square()
    }

    pub fn victim_piece(&self, mv: &Move) -> Option<Piece> {
        if mv.flags().is_en_passant() {
            Some(Piece::Pawn)
        } else if mv.flags().is_capture() {
            for piece in Piece::ALL {
                if self
                    .bitboards
                    .piece(!self.side_to_move(), piece)
                    .get(mv.to())
                {
                    return Some(piece);
                }
            }

            None
        } else {
            None
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    a b c d e f g h")?;
        writeln!(f, "  +{}+", "-".repeat(17))?;

        for rank in (0..8).rev() {
            write!(f, "{} | ", rank + 1)?;

            'outer: for file in 0..8 {
                for color in Color::ALL {
                    for piece in Piece::ALL {
                        if self
                            .bitboards
                            .piece(color, piece)
                            .get(Square::from_rf(rank, file))
                        {
                            write!(f, "{} ", piece.to_char(color))?;
                            continue 'outer;
                        }
                    }
                }

                write!(f, ". ")?;
            }

            writeln!(f, "| {}", rank + 1)?;
        }

        writeln!(f, "  +{}+", "-".repeat(17))?;
        writeln!(f, "    a b c d e f g h")?;
        writeln!(f)?;

        writeln!(f, "Side to move: {:?}", self.side_to_move)?;
        writeln!(f, "Castling: {}", self.castling)?;
        writeln!(
            f,
            "En passant: {}",
            self.en_passant_square
                .map(Square::to_str)
                .unwrap_or("-".to_string())
        )?;
        writeln!(f, "Halfmove clock: {}", self.halfmove_clock)?;
        writeln!(f, "Fullmove number: {}", self.fullmove_number)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fen() {
        let position = Position::from_fen(FEN_INITIAL_POSITION).unwrap();

        assert_eq!(position.side_to_move, Color::White);
        assert_eq!(position.castling, Castling::ALL);
        assert_eq!(position.en_passant_square, None);
    }
}
