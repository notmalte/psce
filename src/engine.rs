use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::{
    game::Game,
    piece::Piece,
    ply::{Ply, PlyKind},
    position::Position,
    square::Square,
};

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

pub fn sort_potential_plies(plies: Vec<Ply>) -> Vec<Ply> {
    let mut plies = plies;

    plies.sort_by(|a, b| {
        let a_kind = match a.kind {
            PlyKind::CastleKingside => 1,
            PlyKind::CastleQueenside => 2,
            PlyKind::PromotionCapture { .. } => 3,
            PlyKind::Promotion { .. } => 4,
            PlyKind::EnPassant { .. } => 5,
            PlyKind::Capture { .. } => 6,
            PlyKind::Regular { .. } => 7,
        };

        let b_kind = match b.kind {
            PlyKind::CastleKingside => 1,
            PlyKind::CastleQueenside => 2,
            PlyKind::PromotionCapture { .. } => 3,
            PlyKind::Promotion { .. } => 4,
            PlyKind::EnPassant { .. } => 5,
            PlyKind::Capture { .. } => 6,
            PlyKind::Regular { .. } => 7,
        };

        if a_kind != b_kind {
            return a_kind.cmp(&b_kind);
        }

        let a_is_capture = match a.kind {
            PlyKind::Capture { .. } | PlyKind::PromotionCapture { .. } => true,
            _ => false,
        };

        let b_is_capture = match b.kind {
            PlyKind::Capture { .. } | PlyKind::PromotionCapture { .. } => true,
            _ => false,
        };

        if a_is_capture && b_is_capture {
            let a_captured = match a.kind {
                PlyKind::Capture { captured, .. } | PlyKind::PromotionCapture { captured, .. } => {
                    captured
                }
                _ => panic!("Not a capture"),
            };

            let b_captured = match b.kind {
                PlyKind::Capture { captured, .. } | PlyKind::PromotionCapture { captured, .. } => {
                    captured
                }
                _ => panic!("Not a capture"),
            };

            let a_captured_value = piece_value(a_captured);
            let b_captured_value = piece_value(b_captured);

            return b_captured_value.cmp(&a_captured_value);
        }

        return std::cmp::Ordering::Equal;
    });

    plies
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
    let opponent_sorted_legal_plies = sort_potential_plies(opponent_legal_plies);

    let mut eval = -CHECKMATE_EVAL;

    let mut local_alpha = alpha;

    for opponent_ply in opponent_sorted_legal_plies {
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
