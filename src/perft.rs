use crate::{movegen::MoveGen, position::Position};

pub fn count_nodes(move_gen: &MoveGen, position: &Position, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    let moves = move_gen.generate_pseudo_legal_moves(position);

    for m in moves {
        let new_pos_opt = position.make_move(move_gen, &m, false);

        if let Some(new_pos) = new_pos_opt {
            nodes += count_nodes(move_gen, &new_pos, depth - 1);
        }
    }

    nodes
}

pub fn run_perft(move_gen: &MoveGen, position: &Position, depth: usize) {
    println!("Running PERFT to depth {}", depth);

    let t_start = std::time::Instant::now();

    let mut total = 0;

    let moves = move_gen.generate_pseudo_legal_moves(position);

    for m in moves {
        let new_pos_opt = position.make_move(move_gen, &m, false);

        if let Some(new_pos) = new_pos_opt {
            let nodes = count_nodes(move_gen, &new_pos, depth - 1);

            total += nodes;

            println!("{}: {}", m, nodes);
        }
    }

    let t_end = std::time::Instant::now();

    println!(
        "Depth: {}, Nodes: {}, Time: {:#.2?}, NPS: {:.0}",
        depth,
        total,
        t_end - t_start,
        total as f64 / (t_end - t_start).as_secs_f64()
    );
}
