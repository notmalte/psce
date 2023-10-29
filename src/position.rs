use std::fmt::Display;

use crate::{
    bitboard::{rf_to_square, square_to_string, string_to_square, BitboardContainer},
    enums::{CastlingRights, Color, PieceAndColor},
};

pub struct Position {
    bitboards: BitboardContainer,
    side_to_move: Color,
    en_passant_square: Option<u8>,
    castling_rights: u8,
}

impl Position {
    pub fn empty() -> Self {
        Self {
            bitboards: BitboardContainer::empty(),
            side_to_move: Color::White,
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

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn side_to_move_mut(&mut self) -> &mut Color {
        &mut self.side_to_move
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
    // initial  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 "
    // tricky   "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 "
    // killer   "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1"
    // cmk      "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 "
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

        position.side_to_move = match fen_parts[1] {
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
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.bitboards)?;

        writeln!(f, "Side to move: \t\t{}", self.side_to_move)?;
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
