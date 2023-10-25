use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::{game::Game, piece::Piece, ply::Ply, position::Position, square::Square};

const CHECKMATE_EVAL: isize = 999_999;

fn piece_value(piece: Piece) -> isize {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight => 3,
        Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 0,
    }
}

fn evaluate_game(game: &Game) -> isize {
    let mut score = 0;

    let self_color = game.color_to_move;
    let opponent_color = self_color.opponent();

    let self_in_check = game.is_in_check(self_color);
    let opponent_in_check = game.is_in_check(opponent_color);

    if opponent_in_check {
        panic!("Opponent already in check");
    }

    let legal_moves = game.find_legal_plies(self_color);

    if legal_moves.len() == 0 {
        if self_in_check {
            return -CHECKMATE_EVAL;
        } else {
            return 0;
        }
    }

    if self_in_check {
        score -= 1;
    }

    for row in 0..=7 {
        for col in 0..=7 {
            let square = game.board.get(Position::new(row, col));

            if let Square::Occupied(color, piece) = square {
                let value = piece_value(piece);

                if color == self_color {
                    score += value;
                } else {
                    score -= value;
                }
            }
        }
    }

    score
}

fn evaluate_ply_with_depth(
    game: &Game,
    ply: Ply,
    depth: usize,
    alpha: isize,
    beta: isize,
) -> isize {
    let mut clone = game.clone();
    clone.make_trusted_ply(ply);

    if depth == 0 {
        return -evaluate_game(&clone);
    }

    let opponent_legal_plies = clone.find_legal_plies(clone.color_to_move);

    let mut eval = -CHECKMATE_EVAL;

    let mut local_alpha = alpha;

    for opponent_ply in opponent_legal_plies {
        eval = eval.max(evaluate_ply_with_depth(
            &clone,
            opponent_ply,
            depth - 1,
            -beta,
            -local_alpha,
        ));

        local_alpha = local_alpha.max(eval);

        if local_alpha >= beta {
            break;
        }
    }

    -eval
}

pub fn find_best_ply(game: &Game, depth: usize) -> (Ply, Vec<(Ply, isize)>) {
    let legal_plies = game.find_legal_plies(game.color_to_move);

    assert!(legal_plies.len() > 0);

    let evals = legal_plies
        .par_iter()
        .map(|ply| evaluate_ply_with_depth(game, *ply, depth, -CHECKMATE_EVAL, CHECKMATE_EVAL))
        .collect::<Vec<isize>>();

    let mut zipped_and_sorted = legal_plies
        .iter()
        .cloned()
        .zip(evals.iter().cloned())
        .collect::<Vec<(Ply, isize)>>();
    zipped_and_sorted.sort_by(|(_, a), (_, b)| b.cmp(a));

    let best_eval = zipped_and_sorted[0].1;

    let best_plies = zipped_and_sorted
        .iter()
        .cloned()
        .filter(|(_, eval)| *eval == best_eval)
        .map(|(ply, _)| ply)
        .collect::<Vec<Ply>>();

    let best_ply = *best_plies.choose(&mut rand::thread_rng()).unwrap();

    (best_ply, zipped_and_sorted)
}
