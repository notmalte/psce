use crate::engine::{movegen::MoveGen, position::Position};

pub fn run() {
    let pos = Position::initial();

    let mg = MoveGen::new();

    println!("{}", pos);

    for m in mg.generate_moves(&pos) {
        println!("{}", m);
    }
}
