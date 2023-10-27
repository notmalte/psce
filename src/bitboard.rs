pub fn get_bit(bitboard: u64, square: u8) -> u64 {
    bitboard & (1u64 << square)
}

pub fn set_bit(bitboard: &mut u64, square: u8) {
    *bitboard |= 1 << square
}

pub fn clear_bit(bitboard: &mut u64, square: u8) {
    *bitboard &= !(1 << square)
}

pub fn toggle_bit(bitboard: &mut u64, square: u8) {
    *bitboard ^= 1 << square
}

pub fn rf_to_square(rank: u8, file: u8) -> u8 {
    rank * 8 + file
}

pub fn rf_to_square_i8(rank: i8, file: i8) -> i8 {
    rank * 8 + file
}

pub fn print_bitboard(bitboard: u64) {
    println!();

    for rank in 0..8 {
        for file in 0..8 {
            let square = rf_to_square(rank, file);

            if file == 0 {
                print!("{} ", 8 - rank);
            }

            print!(" {}", if get_bit(bitboard, square) != 0 { 1 } else { 0 });
        }

        println!();
    }

    println!("\n   a b c d e f g h\n");

    println!("Bitboard: {}\n", bitboard)
}
