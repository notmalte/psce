use core::{Move, MoveFlags, Piece, Position, Square};

use movegen::MoveGen;

fn main() {
    let mut position = Position::initial();
    println!("{}", position);

    position.make_move(&Move::new(
        Square::E2,
        Square::E4,
        Piece::Pawn,
        None,
        MoveFlags::DOUBLE_PUSH,
    ));
    println!("{}", position);

    position.make_move(&Move::new(
        Square::E7,
        Square::E5,
        Piece::Pawn,
        None,
        MoveFlags::DOUBLE_PUSH,
    ));
    println!("{}", position);

    let moves = MoveGen::pseudo_legals(&position);

    println!("{} pseudo-legal moves:", moves.len());
    for m in moves {
        println!("{}", m);
    }
}
