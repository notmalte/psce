use crate::{
    codegen::{generate_and_print_bitboard_constants, generate_and_print_magic_numbers},
    enums::Color,
    movegen::MoveGen,
    position::Position,
    search::search,
};

mod bitboard;
mod codegen;
mod constants;
mod enums;
mod movegen;
mod perft;
mod position;
mod search;

const SEARCH_DEPTH: usize = 6;

fn print_uci_id_and_ok() {
    println!("id name Pretty Solid Chess");
    println!("id author notmalte");
    println!("uciok");
}

fn run_uci(move_gen: &MoveGen) {
    print_uci_id_and_ok();

    let mut position = Position::default();

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "quit" {
            break;
        }

        if input == "uci" {
            print_uci_id_and_ok();
            continue;
        }

        if input == "isready" {
            println!("readyok");
            continue;
        }

        if input == "ucinewgame" {
            position = Position::default();
            continue;
        }

        if input.starts_with("position") {
            let position_opt = Position::from_uci(&move_gen, input.to_string());

            if let Some(new_position) = position_opt {
                position = new_position;
            } else {
                println!("info string Invalid position");
            }

            continue;
        }

        if input == "d" {
            print!("{}", position);
            continue;
        }

        if input.starts_with("go") {
            let (best_eval, best_move) = search(&move_gen, &position, SEARCH_DEPTH);

            if position.color_to_move() == Color::Black {
                println!("info score cp {}", -best_eval);
            } else {
                println!("info score cp {}", best_eval);
            }
            println!("bestmove {}", best_move.unwrap().to_uci());

            continue;
        }
    }
}

fn main() {
    // TODO cli flag
    if false {
        generate_and_print_bitboard_constants();
        println!();
        generate_and_print_magic_numbers();
    }

    let move_gen = MoveGen::new();

    run_uci(&move_gen);
}
