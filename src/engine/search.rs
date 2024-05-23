use crate::engine::{movegen::MoveGen, moves::Move, position::Position};

pub struct SearchResult {
    pub score: isize,
    pub best_move: Move,
}

pub fn search(mg: &MoveGen, pos: &Position, depth: u8) -> Option<SearchResult> {
    let moves = mg.generate_legal_moves(pos);

    if moves.is_empty() {
        return None;
    }

    let random_move = moves[fastrand::usize(..moves.len())].0.clone();

    Some(SearchResult {
        score: 0,
        best_move: random_move,
    })
}
