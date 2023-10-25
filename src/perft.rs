use crate::game::Game;
use rayon::prelude::*;

pub fn perft(game: &Game, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let plies = game.find_legal_plies(game.color_to_move);

    let mut nodes = 0;

    nodes += plies
        .par_iter()
        .map(|ply| {
            let mut clone = game.clone();

            clone.make_trusted_ply(*ply);

            perft(&clone, depth - 1)
        })
        .sum::<usize>();

    nodes
}
