use core::{Move, Position};

use movegen::MoveGen;

mod eval;

pub use eval::evaluate_position;

pub fn find_best_move(position: &Position) -> Option<Move> {
    let legals = MoveGen::legals(position);

    fastrand::choice(legals)
}
