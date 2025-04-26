use core::Position;
use std::time::Instant;

use movegen::MoveGen;

fn perft(pos: &mut Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = MoveGen::pseudo_legals(pos);

    let own_color = pos.side_to_move();

    for m in moves {
        let undo = pos.make_move(&m);

        let king_square = pos.king_square(own_color).expect("should have a king");
        let is_king_attacked = MoveGen::is_attacked(pos, king_square, pos.side_to_move());

        if !is_king_attacked {
            nodes += perft(pos, depth - 1);
        }

        pos.undo_move(&m, &undo);
    }

    nodes
}

fn main() {
    let mut position = Position::initial();

    let depth = 5;

    let start = Instant::now();

    for d in 0..=depth {
        let d_start = Instant::now();
        let nodes = perft(&mut position, d);
        println!(
            "Perft({}) = {} ({:.2}ms)",
            d,
            nodes,
            d_start.elapsed().as_millis()
        );
    }

    println!("Total time: {:.2}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perft_1() {
        let mut position = Position::initial();
        assert_eq!(perft(&mut position, 1), 20);
        assert_eq!(perft(&mut position, 2), 400);
        assert_eq!(perft(&mut position, 3), 8902);
        assert_eq!(perft(&mut position, 4), 197281);
    }

    #[test]
    fn test_perft_2() {
        let mut position = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(perft(&mut position, 1), 48);
        assert_eq!(perft(&mut position, 2), 2039);
        assert_eq!(perft(&mut position, 3), 97862);
    }

    #[test]
    fn test_perft_3() {
        let mut position = Position::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&mut position, 1), 14);
        assert_eq!(perft(&mut position, 2), 191);
        assert_eq!(perft(&mut position, 3), 2812);
        assert_eq!(perft(&mut position, 4), 43238);
        assert_eq!(perft(&mut position, 5), 674624);
    }

    #[test]
    fn test_perft_4() {
        let mut position =
            Position::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
                .unwrap();
        assert_eq!(perft(&mut position, 1), 6);
        assert_eq!(perft(&mut position, 2), 264);
        assert_eq!(perft(&mut position, 3), 9467);
        assert_eq!(perft(&mut position, 4), 422333);
    }

    #[test]
    fn test_perft_5() {
        let mut position =
            Position::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                .unwrap();
        assert_eq!(perft(&mut position, 1), 44);
        assert_eq!(perft(&mut position, 2), 1486);
        assert_eq!(perft(&mut position, 3), 62379);
    }

    #[test]
    fn test_perft_6() {
        let mut position = Position::from_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        )
        .unwrap();
        assert_eq!(perft(&mut position, 1), 46);
        assert_eq!(perft(&mut position, 2), 2079);
        assert_eq!(perft(&mut position, 3), 89890);
    }
}
