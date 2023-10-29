use crate::{
    bitboard::print_bitboard,
    codegen::{generate_and_print_bitboard_constants, generate_and_print_magic_numbers},
    constants::D4,
    movegen::MoveGen,
};

mod bitboard;
mod codegen;
mod constants;
mod enums;
mod movegen;
mod position;

fn main() {
    let t1 = std::time::Instant::now();

    // TODO cli flag
    if false {
        generate_and_print_bitboard_constants();
        println!();
        generate_and_print_magic_numbers();
    }

    let move_gen = MoveGen::new();

    let t2 = std::time::Instant::now();
    println!("Ready (took {:#.2?})", t2 - t1);

    print_bitboard(move_gen.queen().get_attacks(D4, 0u64));
}
