use std::{
    fs::File,
    io::{BufRead, BufReader, Write, stdout},
    time::Instant,
};

use psce_core::Position;
use psce_search::find_best_move;

pub fn run(file: File, depth: u8, count: Option<usize>) {
    let mut lines = parse_file(file);

    if let Some(count) = count {
        lines = sample(lines, count);
    }

    let total = lines.len();
    let mut total_correct = 0;
    let mut total_nodes = 0;
    let mut total_beta_cutoffs = 0;

    println!("Checking {} positions with depth {}", total, depth);

    let start = Instant::now();

    let chunk_size = 20;

    for (i, (pos, ranked_moves)) in lines.iter().enumerate() {
        let engine_result = find_best_move(&pos, depth).expect("No engine result");
        let engine_move = engine_result.pv[0].to_string();

        if ranked_moves[0] == engine_move {
            total_correct += 1;
            print!(". ");
        } else {
            print!("F ");
        }
        stdout().flush().unwrap();

        total_nodes += engine_result.stats.nodes;
        total_beta_cutoffs += engine_result.stats.beta_cutoffs;

        if (i > 0 && (i + 1) % chunk_size == 0) || i == total - 1 {
            println!(
                "# {} - {}",
                if i > chunk_size {
                    i - chunk_size + 2
                } else {
                    1
                },
                i + 1
            );
        }
    }

    let elapsed = start.elapsed();

    println!(
        "{:.2}% correct - {} out of {}",
        (total_correct as f64 / total as f64) * 100.0,
        total_correct,
        total
    );
    println!(
        "{} nodes in {}s ({}nps)",
        total_nodes,
        elapsed.as_secs(),
        (total_nodes as f64 / elapsed.as_secs_f64()) as u64
    );
    println!(
        "{} beta cutoffs ({:.2}%)",
        total_beta_cutoffs,
        (total_beta_cutoffs as f64 / total_nodes as f64 * 100.0)
    );
}

fn sample<T: Clone>(items: Vec<T>, n: usize) -> Vec<T> {
    let total = items.len();

    if n >= total {
        items
    } else {
        (0..n).map(|i| items[(i * total) / n].clone()).collect()
    }
}

fn parse_file(file: File) -> Vec<(Position, Vec<String>)> {
    BufReader::new(file)
        .lines()
        .map(|l| parse_line(&l.expect("Failed to read line")))
        .collect()
}

fn parse_line(line: &str) -> (Position, Vec<String>) {
    let (fen, rest) = line.split_once(" bm ").expect("Invalid line");
    let pos = Position::from_fen(fen).expect("Invalid FEN");

    let (_, attrs) = rest.split_once(";").expect("Invalid line");

    let moves_long = attrs
        .split(";")
        .map(|l| l.trim().split_once(" ").expect("Invalid line"))
        .find_map(|(k, v)| (k == "c9").then_some(v.trim_matches('"')))
        .expect("Invalid line")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    (pos, moves_long)
}
