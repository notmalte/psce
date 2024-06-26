use std::ops::Not;

use strum::Display;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Display, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn to_repr(self) -> u8 {
        self as u8
    }

    pub fn to_usize(self) -> usize {
        self as usize
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
