use psce_core::{Color, Move, Piece, Position};
use psce_movegen::MoveGen;

mod eval;
mod pv;

use eval::CHECKMATE_SCORE;
use pv::PrincipalVariations;

pub use eval::evaluate_position;

#[derive(Default, Debug)]
pub struct SearchStats {
    pub nodes: u64,
    pub beta_cutoffs: u64,
}

#[derive(Debug)]
pub struct SearchResult {
    pub score: i32,
    pub pv: Vec<Move>,
    pub stats: SearchStats,
}

pub fn find_best_move(pos: &Position, depth: u8) -> Option<SearchResult> {
    let mut pos = pos.clone();
    let mut pvs = PrincipalVariations::new();
    let mut stats = SearchStats::default();

    let score = negamax(
        &mut pos,
        depth,
        0,
        -CHECKMATE_SCORE,
        CHECKMATE_SCORE,
        &mut pvs,
        &mut stats,
    );

    let pv = pvs.get_pv(0);

    if !pv.is_empty() {
        Some(SearchResult { score, pv, stats })
    } else {
        None
    }
}

fn negamax(
    pos: &mut Position,
    depth: u8,
    ply: u8,
    mut alpha: i32,
    beta: i32,
    pvs: &mut PrincipalVariations,
    stats: &mut SearchStats,
) -> i32 {
    stats.nodes += 1;

    pvs.clear_ply(ply as usize);

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

    let moves = sort_moves(MoveGen::pseudo_legals(pos), pos);

    for mv in moves {
        let undo = pos.make_move(&mv);

        let king_square = pos.king_square(color).unwrap();
        let is_king_attacked = MoveGen::is_attacked(pos, king_square, opponent);

        if is_king_attacked {
            pos.undo_move(&mv, &undo);
            continue;
        }

        found_legal = true;

        let score = -negamax(pos, depth - 1, ply + 1, -beta, -alpha, pvs, stats);
        pos.undo_move(&mv, &undo);

        if score > best {
            best = score;

            pvs.update(ply as usize, mv);

            if score > alpha {
                alpha = score;
            }
        }

        if alpha >= beta {
            stats.beta_cutoffs += 1;
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

fn sort_moves(mut moves: Vec<Move>, pos: &Position) -> Vec<Move> {
    moves.sort_by_cached_key(|mv| {
        if mv.flags().is_capture() {
            let victim_value = piece_value(pos.victim_piece(mv).unwrap());
            let attacker_value = piece_value(mv.piece());

            (-victim_value, attacker_value)
        } else {
            (0, 0)
        }
    });

    moves
}

fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight => 2,
        Piece::Bishop => 3,
        Piece::Rook => 4,
        Piece::Queen => 5,
        Piece::King => 99,
    }
}
