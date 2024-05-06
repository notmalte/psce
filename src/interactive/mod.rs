use crate::engine::{movegen::MoveGen, position::Position};

pub fn run() {
    let pos = Position::initial();

    let mg = MoveGen::new();

    println!("{}", pos);

    for m in mg.pawn().generate_moves(&pos) {
        println!("{}", m);
    }

    for m in mg.knight().generate_moves(&pos) {
        println!("{}", m);
    }

    for m in mg.bishop().generate_moves(&pos) {
        println!("{}", m);
    }

    for m in mg.rook().generate_moves(&pos) {
        println!("{}", m);
    }

    for m in mg.king().generate_moves(&pos, &mg) {
        println!("{}", m);
    }
}
