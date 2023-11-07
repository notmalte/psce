use clap::{ArgGroup, Parser};
use interactive::run_interactive;
use perft::run_perft;
use uci::run_uci;

use crate::{
    codegen::{generate_and_print_bitboard_constants, generate_and_print_magic_numbers},
    position::Position,
};

mod bitboard;
mod codegen;
mod constants;
mod enums;
mod interactive;
mod movegen;
mod perft;
mod position;
mod search;
mod uci;

#[derive(Parser)]
#[clap(author, version, about = "PSCE - Pretty Solid Chess Engine", long_about = None)]
#[clap(group(ArgGroup::new("mode").args(&["bitboard_constants", "magic_numbers", "uci", "perft"]).required(false)))]
struct Args {
    /// Run interactive mode (default)
    #[clap(long, short, group = "mode")]
    interactive: bool,

    /// Run PERFT benchmark
    #[clap(long, short, value_name = "DEPTH", group = "mode")]
    perft: Option<usize>,

    /// Run in UCI mode
    #[clap(long, short, group = "mode")]
    uci: bool,

    /// Enable logging UCI commands and responses to a file
    #[clap(long, short, requires = "uci", conflicts_with_all = &["interactive", "perft", "bitboard_constants", "magic_numbers"])]
    log_uci: bool,

    /// Generate and print bitboard constants
    #[clap(long, short, group = "mode")]
    bitboard_constants: bool,

    /// Generate and print magic numbers
    #[clap(long, short, group = "mode")]
    magic_numbers: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.bitboard_constants {
        generate_and_print_bitboard_constants();
        return;
    }

    if args.magic_numbers {
        generate_and_print_magic_numbers();
        return;
    }

    if let Some(perft_depth) = args.perft {
        run_perft(&Position::default(), perft_depth);
        return;
    }

    if args.uci {
        run_uci();
        return;
    }

    run_interactive();
}
