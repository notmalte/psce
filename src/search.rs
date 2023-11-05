use strum::IntoEnumIterator;

use crate::{
    bitboard::clear_bit,
    constants::{
        A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, C1, C2, C3, C4, C5, C6, C7,
        C8, D1, D2, D3, D4, D5, D6, D7, D8, E1, E2, E3, E4, E5, E6, E7, E8, F1, F2, F3, F4, F5, F6,
        F7, F8, G1, G2, G3, G4, G5, G6, G7, G8, H1, H2, H3, H4, H5, H6, H7, H8,
    },
    enums::{Color, PieceAndColor},
    movegen::{Move, MoveGen},
    position::Position,
};

#[rustfmt::skip]
const MIRROR_SQUARE: [u8; 64] = [
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
];

#[rustfmt::skip]
const SQUARE_SCORE_PAWN: [isize; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
    50,  50,  50,  50,  50,  50,  50,  50,
    10,  10,  20,  30,  30,  20,  10,  10,
     5,   5,  10,  25,  25,  10,   5,   5,
     0,   0,   0,  20,  20,   0,   0,   0,
     5,  -5, -10,   0,   0, -10,  -5,   5,
     5,  10,  10, -20, -20,  10,  10,   5,
     0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
const SQUARE_SCORE_KNIGHT: [isize; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

#[rustfmt::skip]
const SQUARE_SCORE_BISHOP: [isize; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

#[rustfmt::skip]
const SQUARE_SCORE_ROOK: [isize; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
     5,  10,  10,  10,  10,  10,  10,   5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
     0,   0,   0,   5,   5,   0,   0,   0,
];

#[rustfmt::skip]
const SQUARE_SCORE_QUEEN: [isize; 64] = [
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,   5,   5,   5,   0, -10,
     -5,   0,   5,   5,   5,   5,   0,  -5,
      0,   0,   5,   5,   5,   5,   0,  -5,
    -10,   5,   5,   5,   5,   5,   0, -10,
    -10,   0,   5,   0,   0,   0,   0, -10,
    -20, -10, -10,  -5,  -5, -10, -10, -20,
];

fn material_score(piece: PieceAndColor) -> isize {
    match piece {
        PieceAndColor::WhitePawn => 100,
        PieceAndColor::WhiteKnight => 320,
        PieceAndColor::WhiteBishop => 330,
        PieceAndColor::WhiteRook => 500,
        PieceAndColor::WhiteQueen => 900,
        PieceAndColor::WhiteKing => 20000,
        PieceAndColor::BlackPawn => -100,
        PieceAndColor::BlackKnight => -320,
        PieceAndColor::BlackBishop => -330,
        PieceAndColor::BlackRook => -500,
        PieceAndColor::BlackQueen => -900,
        PieceAndColor::BlackKing => -20000,
    }
}

fn square_score(piece: PieceAndColor, square: u8) -> isize {
    match piece {
        PieceAndColor::WhitePawn => SQUARE_SCORE_PAWN[square as usize],
        PieceAndColor::WhiteKnight => SQUARE_SCORE_KNIGHT[square as usize],
        PieceAndColor::WhiteBishop => SQUARE_SCORE_BISHOP[square as usize],
        PieceAndColor::WhiteRook => SQUARE_SCORE_ROOK[square as usize],
        PieceAndColor::WhiteQueen => SQUARE_SCORE_QUEEN[square as usize],
        PieceAndColor::WhiteKing => 0,
        PieceAndColor::BlackPawn => -SQUARE_SCORE_PAWN[MIRROR_SQUARE[square as usize] as usize],
        PieceAndColor::BlackKnight => -SQUARE_SCORE_KNIGHT[MIRROR_SQUARE[square as usize] as usize],
        PieceAndColor::BlackBishop => -SQUARE_SCORE_BISHOP[MIRROR_SQUARE[square as usize] as usize],
        PieceAndColor::BlackRook => -SQUARE_SCORE_ROOK[MIRROR_SQUARE[square as usize] as usize],
        PieceAndColor::BlackQueen => -SQUARE_SCORE_QUEEN[MIRROR_SQUARE[square as usize] as usize],
        PieceAndColor::BlackKing => 0,
    }
}

fn eval_position(position: &Position) -> isize {
    let mut score = 0;

    for piece in PieceAndColor::iter() {
        let mut bitboard = position.bitboards().piece(piece);

        while bitboard != 0 {
            let square = bitboard.trailing_zeros() as u8;

            score += material_score(piece);
            score += square_score(piece, square);

            clear_bit(&mut bitboard, square);
        }
    }

    match position.color_to_move() {
        Color::White => score,
        Color::Black => -score,
        Color::Both => unreachable!(),
    }
}

const STARTING_EVAL: isize = -100_000;
const CHECKMATE_EVAL: isize = -99_000;

fn quiescence(
    move_gen: &MoveGen,
    position: &Position,
    mut alpha: isize,
    beta: isize,
    ply: usize,
) -> isize {
    let eval = eval_position(position);

    if eval >= beta {
        return beta;
    }

    alpha = alpha.max(eval);

    let pseudo_moves = move_gen.generate_pseudo_legal_moves_sorted(position);

    for pseudo_move in pseudo_moves {
        let new_position_opt = position.make_move(move_gen, &pseudo_move, true);

        if let Some(new_position) = new_position_opt {
            let eval = -quiescence(move_gen, &new_position, -beta, -alpha, ply + 1);

            if eval >= beta {
                return beta;
            }

            alpha = alpha.max(eval);
        }
    }

    alpha
}

fn negamax(
    move_gen: &MoveGen,
    position: &Position,
    depth: usize,
    mut alpha: isize,
    beta: isize,
    ply: usize,
) -> isize {
    if depth == 0 {
        return quiescence(move_gen, position, alpha, beta, ply);
    }

    let pseudo_moves = move_gen.generate_pseudo_legal_moves_sorted(position);

    let mut can_move = false;

    for pseudo_move in pseudo_moves {
        let new_position_opt = position.make_move(move_gen, &pseudo_move, false);

        if let Some(new_position) = new_position_opt {
            can_move = true;

            let eval = -negamax(move_gen, &new_position, depth - 1, -beta, -alpha, ply + 1);

            if eval >= beta {
                return beta;
            }

            alpha = alpha.max(eval);
        }
    }

    if !can_move {
        let king_piece = match position.color_to_move() {
            Color::White => PieceAndColor::WhiteKing,
            Color::Black => PieceAndColor::BlackKing,
            _ => unreachable!(),
        };

        let king_square = position.bitboards().piece(king_piece).trailing_zeros() as u8;

        let king_attacked =
            move_gen.is_square_attacked(position, king_square, !position.color_to_move());

        if king_attacked {
            return CHECKMATE_EVAL + ply as isize;
        } else {
            return 0;
        }
    }

    alpha
}

pub fn search(move_gen: &MoveGen, position: &Position, depth: usize) -> (isize, Option<Move>) {
    let pseudo_moves = move_gen.generate_pseudo_legal_moves_sorted(position);

    let mut best_eval = STARTING_EVAL;
    let mut best_move = None;

    for pseudo_move in pseudo_moves {
        let new_position_opt = position.make_move(move_gen, &pseudo_move, false);

        if let Some(new_position) = new_position_opt {
            let eval = -negamax(
                move_gen,
                &new_position,
                depth - 1,
                best_eval,
                -STARTING_EVAL,
                0,
            );

            if eval > best_eval {
                best_eval = eval;
                best_move = Some(pseudo_move);
            }
        }
    }

    (best_eval, best_move)
}
