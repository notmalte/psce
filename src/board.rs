use colored::Colorize;

use crate::{color::Color, piece::Piece, position::Position, square::Square};

#[derive(Debug, Clone)]
pub struct Board {
    squares: [[Square; 8]; 8],
}

impl Board {
    pub fn new(squares: [[Square; 8]; 8]) -> Self {
        return Self { squares };
    }

    pub fn empty() -> Self {
        return Self::new([[Square::Empty; 8]; 8]);
    }

    pub fn initial() -> Self {
        let mut board = Self::empty();

        for x in 0..=7 {
            board.set(
                Position::new(1, x),
                Square::Occupied(Color::Black, Piece::Pawn),
            );
            board.set(
                Position::new(6, x),
                Square::Occupied(Color::White, Piece::Pawn),
            );
        }

        board.set(
            Position::new(0, 0),
            Square::Occupied(Color::Black, Piece::Rook),
        );
        board.set(
            Position::new(0, 1),
            Square::Occupied(Color::Black, Piece::Knight),
        );
        board.set(
            Position::new(0, 2),
            Square::Occupied(Color::Black, Piece::Bishop),
        );
        board.set(
            Position::new(0, 3),
            Square::Occupied(Color::Black, Piece::Queen),
        );
        board.set(
            Position::new(0, 4),
            Square::Occupied(Color::Black, Piece::King),
        );
        board.set(
            Position::new(0, 5),
            Square::Occupied(Color::Black, Piece::Bishop),
        );
        board.set(
            Position::new(0, 6),
            Square::Occupied(Color::Black, Piece::Knight),
        );
        board.set(
            Position::new(0, 7),
            Square::Occupied(Color::Black, Piece::Rook),
        );

        board.set(
            Position::new(7, 0),
            Square::Occupied(Color::White, Piece::Rook),
        );
        board.set(
            Position::new(7, 1),
            Square::Occupied(Color::White, Piece::Knight),
        );
        board.set(
            Position::new(7, 2),
            Square::Occupied(Color::White, Piece::Bishop),
        );
        board.set(
            Position::new(7, 3),
            Square::Occupied(Color::White, Piece::Queen),
        );
        board.set(
            Position::new(7, 4),
            Square::Occupied(Color::White, Piece::King),
        );
        board.set(
            Position::new(7, 5),
            Square::Occupied(Color::White, Piece::Bishop),
        );
        board.set(
            Position::new(7, 6),
            Square::Occupied(Color::White, Piece::Knight),
        );
        board.set(
            Position::new(7, 7),
            Square::Occupied(Color::White, Piece::Rook),
        );

        board
    }

    pub fn get(&self, position: Position) -> Square {
        return self.squares[position.row][position.col];
    }

    pub fn set(&mut self, position: Position, square: Square) {
        self.squares[position.row][position.col] = square;
    }

    pub fn set_occupied(&mut self, position: Position, color: Color, piece: Piece) {
        self.set(position, Square::Occupied(color, piece));
    }

    pub fn set_empty(&mut self, position: Position) {
        self.set(position, Square::Empty);
    }
}
