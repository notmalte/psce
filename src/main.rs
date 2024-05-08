use clap::Parser;

mod engine;
mod interactive;
mod uci;

/// Pretty Solid Chess Engine
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Run in UCI mode
    #[arg(long)]
    uci: bool,
}

fn main() {
    let args = Args::parse();

    if args.uci {
        uci::run();
    } else {
        interactive::run();
    }
}
