use core::{Move, Position};

use movegen::MoveGen;

pub fn find_best_move(position: &Position) -> Option<Move> {
    let legals = MoveGen::legals(position);

    fastrand::choice(legals)
}
