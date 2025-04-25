use core::Position;
use std::time::Instant;

use movegen::MoveGen;

fn perft(pos: &mut Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = MoveGen::pseudo_legals(pos);

    let own_color = pos.side_to_move();

    for m in moves {
        let undo = pos.make_move(&m);

        let king_square = pos.king_square(own_color).expect("should have a king");
        let is_king_attacked = MoveGen::is_attacked(pos, king_square, pos.side_to_move());

        if !is_king_attacked {
            nodes += perft(pos, depth - 1);
        }

        pos.undo_move(&m, &undo);
    }

    nodes
}

fn main() {
    let mut position = Position::initial();

    let depth = 5;

    let start = Instant::now();

    for d in 0..=depth {
        let d_start = Instant::now();
        let nodes = perft(&mut position, d);
        println!(
            "Perft({}) = {} ({:.2}ms)",
            d,
            nodes,
            d_start.elapsed().as_millis()
        );
    }

    println!("Total time: {:.2}ms", start.elapsed().as_millis());
}
