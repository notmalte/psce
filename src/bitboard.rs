use crate::enums::{Color, PieceAndColor};
use std::fmt::Display;

pub fn get_bit(bitboard: u64, square: u8) -> bool {
    bitboard & (1u64 << square) != 0
}

pub fn set_bit(bitboard: &mut u64, square: u8) {
    *bitboard |= 1 << square
}

pub fn clear_bit(bitboard: &mut u64, square: u8) {
    *bitboard &= !(1 << square)
}

pub fn rf_to_square(rank: u8, file: u8) -> u8 {
    rank * 8 + file
}

pub fn rf_to_square_i8(rank: i8, file: i8) -> i8 {
    rank * 8 + file
}

pub fn square_to_string(square: u8) -> Result<String, String> {
    if square < 64 {
        Ok(format!(
            "{}{}",
            (b'a' + (square % 8)) as char,
            (b'1' + (7 - (square / 8))) as char
        ))
    } else {
        Err("Square out of bounds".to_string())
    }
}

pub fn string_to_square(string: String) -> Result<u8, String> {
    let chars: Vec<_> = string.chars().collect();

    if chars.len() == 2 {
        let chars: Vec<_> = string.chars().collect();

        let file = chars[0];
        let rank = chars[1];

        if ('a'..='h').contains(&file) && ('1'..='8').contains(&rank) {
            Ok(rf_to_square(7 - ((rank as u8) - b'1'), (file as u8) - b'a'))
        } else {
            Err(format!("Invalid square: {}", string))
        }
    } else {
        Err(format!("Invalid square: {}", string))
    }
}

pub fn print_bitboard(bitboard: u64) {
    println!();

    for rank in 0..8 {
        print!("{} ", 8 - rank);

        for file in 0..8 {
            let square = rf_to_square(rank, file);

            print!(" {}", if get_bit(bitboard, square) { "1" } else { "0" });
        }

        println!();
    }

    println!("\n   a b c d e f g h\n");

    println!("Bitboard: {}\n", bitboard)
}

#[derive(Clone)]
pub struct BitboardContainer {
    pieces: [u64; 12],
    colors: [u64; 3],
}

impl BitboardContainer {
    pub fn empty() -> Self {
        Self {
            pieces: [0u64; 12],
            colors: [0u64; 3],
        }
    }

    pub fn piece(&self, piece: PieceAndColor) -> u64 {
        self.pieces[piece as usize]
    }

    pub fn piece_mut(&mut self, piece: PieceAndColor) -> &mut u64 {
        &mut self.pieces[piece as usize]
    }

    pub fn piece_get_bit(&self, piece: PieceAndColor, square: u8) -> bool {
        get_bit(self.pieces[piece as usize], square)
    }

    pub fn piece_set_bit(&mut self, piece: PieceAndColor, square: u8) {
        set_bit(&mut self.pieces[piece as usize], square)
    }

    pub fn piece_clear_bit(&mut self, piece: PieceAndColor, square: u8) {
        clear_bit(&mut self.pieces[piece as usize], square)
    }

    pub fn color(&self, color: Color) -> u64 {
        self.colors[color as usize]
    }

    pub fn color_mut(&mut self, color: Color) -> &mut u64 {
        &mut self.colors[color as usize]
    }

    pub fn color_get_bit(&self, color: Color, square: u8) -> bool {
        get_bit(self.colors[color as usize], square)
    }

    pub fn color_set_bit(&mut self, color: Color, square: u8) {
        set_bit(&mut self.colors[color as usize], square)
    }

    pub fn color_clear_bit(&mut self, color: Color, square: u8) {
        clear_bit(&mut self.colors[color as usize], square)
    }

    pub fn piece_set_bit_incl_color(&mut self, piece: PieceAndColor, square: u8) {
        self.piece_set_bit(piece, square);
        self.color_set_bit(piece.color(), square);
    }

    pub fn piece_clear_bit_incl_color(&mut self, piece: PieceAndColor, square: u8) {
        self.piece_clear_bit(piece, square);
        self.color_clear_bit(piece.color(), square);
    }

    pub fn compute_both(&mut self) {
        self.colors[Color::Both as usize] =
            self.colors[Color::White as usize] | self.colors[Color::Black as usize];
    }
}

impl Display for BitboardContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in 0..8 {
            write!(f, "{} ", 8 - rank)?;

            for file in 0..8 {
                let square = rf_to_square(rank, file);

                let mut piece_opt: Option<PieceAndColor> = None;

                for (piece_index, piece_bitboard) in self.pieces.iter().enumerate() {
                    if get_bit(*piece_bitboard, square) {
                        piece_opt = Some(PieceAndColor::from_repr(piece_index).unwrap());
                        break;
                    }
                }

                if let Some(piece) = piece_opt {
                    write!(f, " {}", piece)?;
                } else {
                    write!(f, " .")?;
                }
            }

            writeln!(f)?;
        }

        writeln!(f, "\n   a b c d e f g h")?;

        Ok(())
    }
}
