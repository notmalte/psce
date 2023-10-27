use crate::{
    bitboard::print_bitboard,
    codegen::{generate_and_print_bitboard_constants, generate_and_print_magic_numbers},
    constants::{B6, C5, D4, E4, E5, F6},
    movegen::{BishopMoveGen, KingMoveGen, KnightMoveGen, PawnMoveGen, RookMoveGen},
};

mod bitboard;
mod codegen;
mod constants;
mod enums;
mod movegen;

fn main() {
    // TODO cli flag
    if false {
        generate_and_print_bitboard_constants();
        println!();
        generate_and_print_magic_numbers();
    }

    let t1 = std::time::Instant::now();

    let pawn_move_gen = PawnMoveGen::new();
    let knight_move_gen = KnightMoveGen::new();
    let king_move_gen = KingMoveGen::new();
    let rook_move_gen = RookMoveGen::new();
    let bishop_move_gen = BishopMoveGen::new();

    let t2 = std::time::Instant::now();

    println!("Ready (took {:#.2?})", t2 - t1);
}
