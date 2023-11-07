use tokio::task;

use crate::{enums::Color, movegen::MoveGen, position::Position, search::search};

const SEARCH_DEPTH: usize = 6;

fn print_uci_id_and_ok() {
    println!("id name Pretty Solid Chess Engine");
    println!("id author notmalte");
    println!("uciok");
}

fn run_search(move_gen: &MoveGen, position: &Position, depth: usize) {
    let (best_eval, best_move) = search(move_gen, position, depth);

    if position.color_to_move() == Color::Black {
        println!("info score cp {}", -best_eval);
    } else {
        println!("info score cp {}", best_eval);
    }
    println!("bestmove {}", best_move.unwrap().to_uci());
}

pub fn run_uci() {
    print_uci_id_and_ok();

    let mut position = Position::default();

    let move_gen = MoveGen::new();

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
            let move_gen_clone = move_gen.clone();
            let position_clone = position.clone();

            task::spawn(async move {
                run_search(&move_gen_clone, &position_clone, SEARCH_DEPTH);
            });

            continue;
        }

        println!("info string Unknown command: {}", input);
    }
}
