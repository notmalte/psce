use core::Position;
use std::io;

use movegen::MoveGen;
use search::find_best_move;

fn main() {
    let mut position = Position::initial();
    println!("{}", position);

    loop {
        let legal_moves = MoveGen::legals(&position);
        if legal_moves.is_empty() {
            println!("Checkmate! {:?} wins!", !position.side_to_move());
            break;
        }

        let mut selected = None;

        loop {
            println!("Enter move (e.g. e2e4):");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let trimmed = input.trim();
            if trimmed == "q" || trimmed == "quit" {
                break;
            }

            let mv = legal_moves.iter().find(|m| m.to_string() == trimmed);
            if let Some(mv) = mv {
                selected = Some(mv);
                break;
            } else {
                println!("Invalid move!");
            }
        }

        let Some(mv) = selected else {
            println!("Bye!");
            break;
        };

        position.make_move(mv);
        println!("{}", position);

        let Some(engine_move) = find_best_move(&position) else {
            println!("Checkmate! {:?} wins!", !position.side_to_move());
            break;
        };

        println!("Engine move: {}", engine_move);

        position.make_move(&engine_move);
        println!("{}", position);
    }
}
