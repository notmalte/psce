use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let command = line.unwrap_or_else(|_| "quit".to_string());

        let parts = command.split_whitespace().collect::<Vec<_>>();

        match parts.get(0) {
            Some(&"uci") => {
                println!("id name psce");
                println!("uciok");
            }
            Some(&"isready") => {
                println!("readyok");
            }
            Some(&"ucinewgame") => {
                // TODO: reset

                println!("uciok");
            }
            Some(&"stop") => {
                // TODO: stop
            }
            Some(&"quit") => {
                // TODO: quit

                break;
            }
            _ => {
                println!("info string unknown command: {}", command);
            }
        }
    }
}
