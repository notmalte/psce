use psce::{
    engine::{
        movegen::MoveGen,
        position::{
            Position, FEN_TEST_POSITION_1, FEN_TEST_POSITION_2, FEN_TEST_POSITION_3,
            FEN_TEST_POSITION_4, FEN_TEST_POSITION_5,
        },
    },
    perft::count_nodes,
};

#[test]
fn perft_initial_position() {
    let mg = MoveGen::new();
    let pos = Position::initial();

    assert_eq!(count_nodes(&mg, &pos, 1), 20);
    assert_eq!(count_nodes(&mg, &pos, 2), 400);
    assert_eq!(count_nodes(&mg, &pos, 3), 8902);
    assert_eq!(count_nodes(&mg, &pos, 4), 197281);
}

#[test]
fn perft_test_position_1() {
    let mg = MoveGen::new();
    let pos = Position::from_fen(FEN_TEST_POSITION_1).unwrap();

    assert_eq!(count_nodes(&mg, &pos, 1), 48);
    assert_eq!(count_nodes(&mg, &pos, 2), 2039);
    assert_eq!(count_nodes(&mg, &pos, 3), 97862);
    assert_eq!(count_nodes(&mg, &pos, 4), 4085603);
}

#[test]
fn perft_test_position_2() {
    let mg = MoveGen::new();
    let pos = Position::from_fen(FEN_TEST_POSITION_2).unwrap();

    assert_eq!(count_nodes(&mg, &pos, 1), 14);
    assert_eq!(count_nodes(&mg, &pos, 2), 191);
    assert_eq!(count_nodes(&mg, &pos, 3), 2812);
    assert_eq!(count_nodes(&mg, &pos, 4), 43238);
}

#[test]
fn perft_test_position_3() {
    let mg = MoveGen::new();
    let pos = Position::from_fen(FEN_TEST_POSITION_3).unwrap();

    assert_eq!(count_nodes(&mg, &pos, 1), 6);
    assert_eq!(count_nodes(&mg, &pos, 2), 264);
    assert_eq!(count_nodes(&mg, &pos, 3), 9467);
    assert_eq!(count_nodes(&mg, &pos, 4), 422333);
}

#[test]
fn perft_test_position_4() {
    let mg = MoveGen::new();
    let pos = Position::from_fen(FEN_TEST_POSITION_4).unwrap();

    assert_eq!(count_nodes(&mg, &pos, 1), 44);
    assert_eq!(count_nodes(&mg, &pos, 2), 1486);
    assert_eq!(count_nodes(&mg, &pos, 3), 62379);
    assert_eq!(count_nodes(&mg, &pos, 4), 2103487);
}

#[test]
fn perft_test_position_5() {
    let mg = MoveGen::new();
    let pos = Position::from_fen(FEN_TEST_POSITION_5).unwrap();

    assert_eq!(count_nodes(&mg, &pos, 1), 46);
    assert_eq!(count_nodes(&mg, &pos, 2), 2079);
    assert_eq!(count_nodes(&mg, &pos, 3), 89890);
    assert_eq!(count_nodes(&mg, &pos, 4), 3894594);
}
