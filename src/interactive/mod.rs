use crate::engine::{
    bitboard::Bitboard,
    castling::{Castling, CastlingRights},
};

pub fn run() {
    let bb = Bitboard::empty();

    let mut cstl = CastlingRights::all();

    cstl.clear(Castling::WhiteKingSide);

    println!("{}", cstl);

    println!("{}", bb)
}
