use std::time::Instant;

use crate::engine::{movegen::MoveGen, position::Position};

pub fn run(depth: u8) {
    println!("Preparing move generator...");

    let mg = MoveGen::new();

    println!("Running perft to depth {}", depth);

    for d in 1..=depth {
        let pos = Position::initial();

        let start = Instant::now();
        let nodes = perft(&mg, &pos, d);
        let duration = start.elapsed();

        let nps = (nodes as f64 / duration.as_secs_f64()) as u64;

        println!(
            "Depth: {}, Nodes: {}, Time: {:?}, NPS: {}",
            d, nodes, duration, nps
        );
    }
}

fn perft(mg: &MoveGen, pos: &Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    for (_, p) in mg.generate_legal_moves(pos) {
        nodes += perft(mg, &p, depth - 1);
    }

    nodes
}
