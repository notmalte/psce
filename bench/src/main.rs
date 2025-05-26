use std::fs::File;

use clap::{Parser, Subcommand};
use psce_core::Position;

mod perft;
mod suite;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Perft {
        #[arg(short, long)]
        fen: Option<String>,
        #[arg(short, long, default_value = "5")]
        depth: u8,
    },
    Suite {
        #[arg(short, long)]
        file: String,
        #[arg(short, long, default_value = "6")]
        depth: u8,
        #[arg(short = 'n', long)]
        count: Option<usize>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Perft { fen, depth } => {
            let pos = fen.map_or(Position::initial(), |fen| {
                Position::from_fen(&fen).expect("Invalid FEN")
            });

            perft::run(pos, depth);
        }
        Command::Suite { file, depth, count } => {
            let file = File::open(file).expect("Failed to open file");

            suite::run(file, depth, count);
        }
    }
}
