use core::{Move, MoveFlags, Piece, Position, Square};

fn main() {
    let mut position = Position::initial();
    println!("{}", position);

    let mv = Move::new(
        Square::E2,
        Square::E4,
        Piece::Pawn,
        None,
        MoveFlags::DOUBLE_PUSH,
    );
    let undo = position.make_move(&mv);
    println!("{}", position);

    position.undo_move(&mv, &undo);
    println!("{}", position);
}
