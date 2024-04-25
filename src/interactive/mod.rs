use crate::engine::{movegen::MoveGen, position::Position};

pub fn run() {
    let pos = Position::initial();

    let mg = MoveGen::new();

    let moves = mg.knight().generate_moves(&pos);

    println!("{}", pos);

    for m in moves {
        println!("{}", m);
    }
}
