use core::{Color, Piece, Position};

pub fn evaluate_position(position: &Position) -> i32 {
    let mut score = 0;

    for color in Color::ALL {
        for piece in Piece::ALL {
            let bitboard = position.bitboards().piece(color, piece);

            for _ in bitboard.squares() {
                if color == Color::White {
                    score += material_score(piece);
                } else {
                    score -= material_score(piece);
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
