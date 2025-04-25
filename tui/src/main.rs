use core::{Move, MoveFlags, Piece, Position, Square};

use movegen::MoveGen;

fn main() {
    let mut position = Position::initial();
    println!("{}", position);

    position.make_move(&Move::new(
        Square::F2,
        Square::F3,
        Piece::Pawn,
        None,
        MoveFlags::NONE,
    ));
    println!("{}", position);

    position.make_move(&Move::new(
        Square::E7,
        Square::E6,
        Piece::Pawn,
        None,
        MoveFlags::NONE,
    ));
    println!("{}", position);

    position.make_move(&Move::new(
        Square::G2,
        Square::G4,
        Piece::Pawn,
        None,
        MoveFlags::DOUBLE_PUSH,
    ));
    println!("{}", position);

    position.make_move(&Move::new(
        Square::D8,
        Square::H4,
        Piece::Queen,
        None,
        MoveFlags::NONE,
    ));
    println!("{}", position);

    let moves = MoveGen::legals(&position);

    println!("{} legal moves:", moves.len());
    for m in moves {
        println!("{}", m);
    }
}
