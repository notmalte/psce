use clap::Parser;

mod engine;
mod interactive;
mod perft;
mod uci;

/// Pretty Solid Chess Engine
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Run in UCI mode
    #[arg(long, group = "mode")]
    uci: bool,

    /// Run PERFT tests
    #[arg(long, value_name = "DEPTH", group = "mode")]
    perft: Option<u8>,
}

fn main() {
    let args = Args::parse();

    if args.uci {
        uci::run();
    } else if let Some(depth) = args.perft {
        perft::run(depth);
    } else {
        interactive::run();
    }
}
