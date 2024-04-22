use std::fmt::Display;

use crate::engine::{
    bitboard::{Bitboard, BitboardContainer},
    castling::CastlingRights,
    color::Color,
    piece::Piece,
};

const FEN_INITIAL_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Position {
    bitboards: BitboardContainer,
    color_to_move: Color,
    castling_rights: CastlingRights,
    en_passant_square: Option<u8>,
}

impl Position {
    pub fn empty() -> Self {
        Self {
            bitboards: BitboardContainer::empty(),
            color_to_move: Color::White,
            castling_rights: CastlingRights::none(),
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

                    let square = Bitboard::xy_to_index(x as u8, y as u8);

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

        position.castling_rights = CastlingRights::from_str(parts[2])?;

        position.en_passant_square = match parts[3] {
            "-" => None,
            _s => unimplemented!(), // TODO
        };

        *position.bitboards.all_mut() =
            position.bitboards.color(Color::White) | position.bitboards.color(Color::Black);

        Ok(position)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n", self.bitboards)?;

        writeln!(f, "Side to move: {}", self.color_to_move)?;
        writeln!(f, "Castling rights: {}", self.castling_rights)?;
        write!(
            f,
            "En passant square: {}",
            match self.en_passant_square {
                Some(square) => square.to_string(), // TODO: still an u8
                None => "-".to_string(),
            }
        )?;

        Ok(())
    }
}
