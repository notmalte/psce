use std::time::Instant;

use rayon::prelude::*;

use crate::engine::{movegen::MoveGen, position::Position};

pub fn run(depth: u8) {
    println!("Preparing move generator...");

    let mg = MoveGen::new();

    println!("Running PERFT to depth {}", depth);

    for d in 1..=depth {
        let pos = Position::initial();

        let start = Instant::now();
        let nodes = count_nodes(&mg, &pos, d);
        let duration = start.elapsed();

        let nps = (nodes as f64 / duration.as_secs_f64()) as u64;

        println!(
            "Depth: {}, Nodes: {}, Time: {:?}, NPS: {}",
            d, nodes, duration, nps
        );
    }
}

pub fn count_nodes(mg: &MoveGen, pos: &Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = mg.generate_legal_moves(pos);

    if depth < 4 {
        moves
            .iter()
            .map(|(_, p)| count_nodes(mg, p, depth - 1))
            .sum()
    } else {
        moves
            .par_iter()
            .map(|(_, p)| count_nodes(mg, p, depth - 1))
            .sum()
    }
}
