use core::Position;

use movegen::MoveGen;

fn main() {
    let mg = MoveGen::new();

    let mut position = Position::initial();
    println!("{}", position);

    let moves = mg.pseudo_legals(&position);

    println!("{} pseudo-legal moves:", moves.len());
    for m in moves {
        println!("{}", m);
    }
}
