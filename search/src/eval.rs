use psce_core::{Color, Piece, Position, Square};

pub const CHECKMATE_SCORE: i32 = 1_000_000;

pub fn evaluate_position(position: &Position) -> i32 {
    let mut score = 0;

    for color in Color::ALL {
        for piece in Piece::ALL {
            let bitboard = position.bitboards().piece(color, piece);

            for square in bitboard.squares() {
                if color == Color::White {
                    score += material_score(piece) + square_score(piece, square);
                } else {
                    score -= material_score(piece) + square_score(piece, Square::mirror(square));
                }
            }
        }
    }

    score
}

fn material_score(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 10000,
    }
}

fn square_score(piece: Piece, square: u8) -> i32 {
    match piece {
        Piece::Pawn => [
            0, 0, 0, 0, 0, 0, 0, 0, //
            50, 50, 50, 50, 50, 50, 50, 50, //
            10, 10, 20, 30, 30, 20, 10, 10, //
            5, 5, 10, 25, 25, 10, 5, 5, //
            0, 0, 0, 20, 20, 0, 0, 0, //
            5, -5, -10, 0, 0, -10, -5, 5, //
            5, 10, 10, -20, -20, 10, 10, 5, //
            0, 0, 0, 0, 0, 0, 0, 0, //
        ][square as usize],
        Piece::Knight => [
            -50, -40, -30, -30, -30, -30, -40, -50, //
            -40, -20, 0, 0, 0, 0, -20, -40, //
            -30, 0, 10, 15, 15, 10, 0, -30, //
            -30, 5, 15, 20, 20, 15, 5, -30, //
            -30, 0, 15, 20, 20, 15, 0, -30, //
            -30, 5, 10, 15, 15, 10, 5, -30, //
            -40, -20, 0, 5, 5, 0, -20, -40, //
            -50, -40, -30, -30, -30, -30, -40, -50, //
        ][square as usize],
        Piece::Bishop => [
            -20, -10, -10, -10, -10, -10, -10, -20, //
            -10, 0, 0, 0, 0, 0, 0, -10, //
            -10, 0, 5, 10, 10, 5, 0, -10, //
            -10, 5, 5, 10, 10, 5, 5, -10, //
            -10, 0, 10, 10, 10, 10, 0, -10, //
            -10, 10, 10, 10, 10, 10, 10, -10, //
            -10, 5, 0, 0, 0, 0, 5, -10, //
            -20, -10, -10, -10, -10, -10, -10, -20, //
        ][square as usize],
        Piece::Rook => [
            0, 0, 0, 0, 0, 0, 0, 0, //
            5, 10, 10, 10, 10, 10, 10, 5, //
            -5, 0, 0, 0, 0, 0, 0, -5, //
            -5, 0, 0, 0, 0, 0, 0, -5, //
            -5, 0, 0, 0, 0, 0, 0, -5, //
            -5, 0, 0, 0, 0, 0, 0, -5, //
            -5, 0, 0, 0, 0, 0, 0, -5, //
            0, 0, 0, 5, 5, 0, 0, 0, //
        ][square as usize],
        Piece::Queen => [
            -20, -10, -10, -5, -5, -10, -10, -20, //
            -10, 0, 0, 0, 0, 0, 0, -10, //
            -10, 0, 5, 5, 5, 5, 0, -10, //
            -5, 0, 5, 5, 5, 5, 0, -5, //
            0, 0, 5, 5, 5, 5, 0, -5, //
            -10, 5, 5, 5, 5, 5, 0, -10, //
            -10, 0, 5, 0, 0, 0, 0, -10, //
            -20, -10, -10, -5, -5, -10, -10, -20, //
        ][square as usize],
        Piece::King => 0,
    }
}
