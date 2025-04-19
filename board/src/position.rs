use std::fmt::Display;

use strum::IntoEnumIterator;

use crate::{
    bitboard::{BitboardContainer, sq, sq_from_str},
    castling::Castling,
    color::Color,
    piece::Piece,
};

pub const FEN_INITIAL_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Position {
    bitboards: BitboardContainer,
    side_to_move: Color,
    castling: Castling,
    en_passant_square: Option<u8>,
    // TODO: counters for halfmove clock and fullmove number
}

impl Position {
    pub fn empty() -> Self {
        Self {
            bitboards: BitboardContainer::empty(),
            side_to_move: Color::White,
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
                    let color = if c.is_lowercase() {
                        Color::Black
                    } else {
                        Color::White
                    };

                    let piece = Piece::from_char(c).ok_or("Invalid piece".to_string())?;

                    let square = sq(y as u8, x as u8);

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
            s => Some(sq_from_str(s).ok_or("Invalid en passant square".to_string())?),
        };

        Ok(position)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    a b c d e f g h")?;
        writeln!(f, "  +{}+", "-".repeat(17))?;

        for rank in (0..8).rev() {
            write!(f, "{} | ", rank + 1)?;

            'outer: for file in 0..8 {
                for color in Color::iter() {
                    for piece in Piece::iter() {
                        if self.bitboards.piece(color, piece).get(sq(rank, file)) {
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
