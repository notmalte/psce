use core::{Piece, Position};

use movegen::MoveGen;

fn perft(pos: &mut Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = MoveGen::pseudo_legals(pos);

    let side = pos.side_to_move();

    for m in moves {
        let undo = pos.make_move(&m);

        let king_square = pos
            .bitboards()
            .piece(side, Piece::King)
            .last_square()
            .unwrap();
        let is_king_attacked = MoveGen::is_attacked(pos, king_square, side);

        if !is_king_attacked {
            nodes += perft(pos, depth - 1);
        }

        pos.undo_move(&m, &undo);
    }

    nodes
}

fn main() {
    let mut position = Position::initial();

    let depth = 3;
    let nodes = perft(&mut position, depth);
    println!("Perft({}) = {}", depth, nodes);
}
