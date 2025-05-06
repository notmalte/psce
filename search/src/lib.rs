use core::{Color, Move, Position};

use eval::CHECKMATE_SCORE;
use movegen::MoveGen;

mod eval;

pub use eval::evaluate_position;

const DEPTH: u8 = 6;

pub fn find_best_move(pos: &Position) -> Option<(i32, Move)> {
    let mut pos = pos.clone();

    let color = pos.side_to_move();
    let opponent = !color;

    let mut best = None;

    let moves = MoveGen::pseudo_legals(&pos);

    for mv in moves {
        let undo = pos.make_move(&mv);

        let king_square = pos.king_square(color).unwrap();
        let is_king_attacked = MoveGen::is_attacked(&pos, king_square, opponent);

        if is_king_attacked {
            pos.undo_move(&mv, &undo);
            continue;
        }

        let score = -negamax(&mut pos, DEPTH, 0, -CHECKMATE_SCORE, CHECKMATE_SCORE);

        if best.as_ref().is_none_or(|(e, _)| score > *e) {
            best = Some((score, mv.clone()));
        }

        pos.undo_move(&mv, &undo);
    }

    best
}

fn negamax(pos: &mut Position, depth: u8, ply: u8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return if pos.side_to_move() == Color::White {
            evaluate_position(pos)
        } else {
            -evaluate_position(pos)
        };
    }

    let color = pos.side_to_move();
    let opponent = !color;

    let mut best = -CHECKMATE_SCORE;
    let mut found_legal = false;

    let moves = MoveGen::pseudo_legals(pos);

    for mv in moves {
        let undo = pos.make_move(&mv);

        let king_square = pos.king_square(color).unwrap();
        let is_king_attacked = MoveGen::is_attacked(pos, king_square, opponent);

        if is_king_attacked {
            pos.undo_move(&mv, &undo);
            continue;
        }

        found_legal = true;

        let score = -negamax(pos, depth - 1, ply + 1, -beta, -alpha);
        pos.undo_move(&mv, &undo);

        if score > best {
            best = score;
        }

        if score > alpha {
            alpha = score;
        }

        if alpha >= beta {
            break;
        }
    }

    if !found_legal {
        let king_square = pos.king_square(color).unwrap();
        let is_king_attacked = MoveGen::is_attacked(pos, king_square, opponent);

        if is_king_attacked {
            return -CHECKMATE_SCORE + ply as i32;
        } else {
            return 0;
        }
    }

    best
}
