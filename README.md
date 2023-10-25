# Chess Engine

Simple chess engine written in Rust, able to beat beginners and Stockfish level 1.

## Usage

```bash
cargo build --release
./target/release/chess
```

## Features

- Move generation for all legal moves
- PERFT validation
- FEN parsing
- Multi-threaded Minimax search up to depth X
- Alpha-beta pruning
- Simple move ordering heuristics
- Simple material-based evaluation at depth 0

There are many possible improvements to be made, as most parts of the engine are not focused on performance. The main goal was to get a working engine with a rule-compliant move generator and a simple evaluation function.
