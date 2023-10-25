use crate::{
    color::Color, engine::find_best_ply, game::Game, outcome::Outcome, ply::Ply, position::Position,
};
use colored::Colorize;

mod attack;
mod board;
mod castling;
mod color;
mod engine;
mod game;
mod outcome;
mod perft;
mod piece;
mod ply;
mod position;
mod square;

const PERFT_DEPTH: usize = 6;
const ENGINE_DEPTH: usize = 5;

fn trusted_input_to_positions(s: String) -> (Position, Position) {
    let (s_from, s_to) = s.split_at(2);

    let from = Position::from_chess(s_from);
    let to = Position::from_chess(s_to);

    (from.unwrap(), to.unwrap())
}

fn find_plies_by_positions(plies: &Vec<Ply>, from: Position, to: Position) -> Vec<Ply> {
    let mut found_plies = vec![];

    for p in plies {
        if p.get_from() == from && p.get_to() == to {
            found_plies.push(*p);
        }
    }

    found_plies
}

fn ask_for_user_ply(legal_plies: &Vec<Ply>) -> Option<Ply> {
    loop {
        let ply_input = inquire::Text::new("Enter ply: ")
            .with_placeholder("e.g. e2e4")
            .with_validator(|s: &str| {
                if s.len() != 4 {
                    return Ok(inquire::validator::Validation::Invalid(
                        "Input must be 4 characters long".into(),
                    ));
                }

                let (s_from, s_to) = s.split_at(2);

                let from = Position::from_chess(s_from);
                let to = Position::from_chess(s_to);

                if from.is_none() {
                    return Ok(inquire::validator::Validation::Invalid(
                        format!("Invalid square: {}", s_from).into(),
                    ));
                }

                if to.is_none() {
                    return Ok(inquire::validator::Validation::Invalid(
                        format!("Invalid square: {}", s_to).into(),
                    ));
                }

                Ok(inquire::validator::Validation::Valid)
            })
            .prompt();

        if let Err(_) = ply_input {
            return None;
        }

        let (from, to) = trusted_input_to_positions(ply_input.unwrap());
        let found_plies = find_plies_by_positions(legal_plies, from, to);

        if found_plies.is_empty() {
            println!("{}", ("# Illegal ply").bright_red());
            continue;
        }

        if found_plies.len() > 1 {
            println!("{}", ("# Ambiguous ply").bright_red());
            continue;
        }

        return Some(found_plies[0]);
    }
}

fn play_manually() {
    let mut game = Game::new();

    loop {
        game.print_board();

        let plies = game.find_legal_plies(game.color_to_move);

        if plies.len() == 0 {
            let in_check = game.is_in_check(game.color_to_move);

            if in_check {
                game.outcome = Some(Outcome::Checkmate {
                    winner: game.color_to_move.opponent(),
                });
            } else {
                game.outcome = Some(Outcome::Stalemate);
            }

            println!("Plies:");

            for (i, p) in game.plies.iter().enumerate() {
                println!("{}. [{}] {}", i + 1, p.color.to_string(), p.to_string());
            }

            println!("{}", game.outcome.unwrap().to_string().bright_green());

            break;
        }

        game.print_captured_material();
        game.print_in_check();
        game.print_color_to_move();
        println!();

        let user_ply = ask_for_user_ply(&plies);

        if user_ply.is_none() {
            println!("Bye!");
            break;
        }

        let user_ply = user_ply.unwrap();

        println!("Ply: {}", user_ply.to_string());

        game.make_trusted_ply(user_ply);
    }
}

fn play_against_engine() {
    let mut game = Game::new();

    let engine_color = Color::Black;

    loop {
        game.print_board();

        let plies = game.find_legal_plies(game.color_to_move);

        if plies.len() == 0 {
            let in_check = game.is_in_check(game.color_to_move);

            if in_check {
                game.outcome = Some(Outcome::Checkmate {
                    winner: game.color_to_move.opponent(),
                });
            } else {
                game.outcome = Some(Outcome::Stalemate);
            }

            println!("Plies:");

            for (i, p) in game.plies.iter().enumerate() {
                println!("{}. [{}] {}", i + 1, p.color.to_string(), p.to_string());
            }

            println!("{}", game.outcome.unwrap().to_string().bright_green());

            break;
        }

        game.print_captured_material();
        game.print_in_check();
        game.print_color_to_move();
        println!();

        if game.color_to_move == engine_color {
            let t1 = std::time::Instant::now();

            let (engine_ply, engine_evals) = find_best_ply(&game, ENGINE_DEPTH);

            let t2 = std::time::Instant::now();

            let max_evals = 3;

            println!(
                "Top {} engine plies ({} evaluated in {} ms):",
                max_evals,
                engine_evals.len(),
                (t2 - t1).as_millis()
            );
            for i in 0..engine_evals.len().min(max_evals) {
                println!(
                    "{}. [{}] {}",
                    i + 1,
                    engine_evals[i].1,
                    engine_evals[i].0.to_string()
                );
            }

            println!("Engine ply: {}", engine_ply.to_string());

            game.make_trusted_ply(engine_ply);

            continue;
        } else {
            let user_ply = ask_for_user_ply(&plies);

            if user_ply.is_none() {
                println!("Bye!");
                break;
            }

            let user_ply = user_ply.unwrap();

            println!("Ply: {}", user_ply.to_string());

            game.make_trusted_ply(user_ply);
        }
    }
}

fn parse_and_display_fen() {
    let fen = inquire::Text::new("Enter FEN: ")
        .with_placeholder("e.g. rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .prompt();

    if let Err(_) = fen {
        println!("Bye!");
        return;
    }

    let fen = fen.unwrap();

    let game = Game::from_fen(fen);

    if let Err(e) = game {
        println!("{}", format!("# {}", e).bright_red());
        return;
    }

    let game = game.unwrap();

    game.print_board();
    game.print_in_check();
    game.print_color_to_move();
}

fn run_perft() {
    let game = Game::new();

    for d in 0..=PERFT_DEPTH {
        let p = perft::perft(&game, d);

        println!("PERFT({}) = {}", d, p);
    }
}

fn run_perft_from_fen() {
    let fen = inquire::Text::new("Enter FEN: ")
        .with_placeholder("e.g. rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .prompt();

    if let Err(_) = fen {
        println!("Bye!");
        return;
    }

    let fen = fen.unwrap();

    let game = Game::from_fen(fen);

    if let Err(e) = game {
        println!("{}", format!("# {}", e).bright_red());
        return;
    }

    let game = game.unwrap();

    for d in 0..=PERFT_DEPTH {
        let p = perft::perft(&game, d);

        println!("PERFT({}) = {}", d, p);
    }
}

fn main() {
    let menu_options = vec![
        ("Play against engine", play_against_engine as fn()),
        ("Play manually", play_manually as fn()),
        ("Parse and display FEN", parse_and_display_fen as fn()),
        ("Run PERFT", run_perft as fn()),
        ("Run PERFT from FEN", run_perft_from_fen as fn()),
    ];

    let answer = inquire::Select::new(
        "Select mode:",
        menu_options.iter().map(|(s, _)| *s).collect(),
    )
    .without_help_message()
    .prompt();

    if answer.is_err() {
        println!("Bye!");
        return;
    }

    let answer = answer.unwrap();

    menu_options.iter().find(|(s, _)| *s == answer).unwrap().1();
}
