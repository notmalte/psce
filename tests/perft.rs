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
fn perft() {
    let mg = MoveGen::new();

    let pos_initial = Position::initial();

    assert_eq!(count_nodes(&mg, &pos_initial, 1), 20);
    assert_eq!(count_nodes(&mg, &pos_initial, 2), 400);
    assert_eq!(count_nodes(&mg, &pos_initial, 3), 8902);
    assert_eq!(count_nodes(&mg, &pos_initial, 4), 197281);
    assert_eq!(count_nodes(&mg, &pos_initial, 5), 4865609);

    let pos_test1 = Position::from_fen(FEN_TEST_POSITION_1).unwrap();

    assert_eq!(count_nodes(&mg, &pos_test1, 1), 48);
    assert_eq!(count_nodes(&mg, &pos_test1, 2), 2039);
    assert_eq!(count_nodes(&mg, &pos_test1, 3), 97862);
    assert_eq!(count_nodes(&mg, &pos_test1, 4), 4085603);
    assert_eq!(count_nodes(&mg, &pos_test1, 5), 193690690);

    let pos_test2 = Position::from_fen(FEN_TEST_POSITION_2).unwrap();

    assert_eq!(count_nodes(&mg, &pos_test2, 1), 14);
    assert_eq!(count_nodes(&mg, &pos_test2, 2), 191);
    assert_eq!(count_nodes(&mg, &pos_test2, 3), 2812);
    assert_eq!(count_nodes(&mg, &pos_test2, 4), 43238);
    assert_eq!(count_nodes(&mg, &pos_test2, 5), 674624);

    let pos_test3 = Position::from_fen(FEN_TEST_POSITION_3).unwrap();

    assert_eq!(count_nodes(&mg, &pos_test3, 1), 6);
    assert_eq!(count_nodes(&mg, &pos_test3, 2), 264);
    assert_eq!(count_nodes(&mg, &pos_test3, 3), 9467);
    assert_eq!(count_nodes(&mg, &pos_test3, 4), 422333);
    assert_eq!(count_nodes(&mg, &pos_test3, 5), 15833292);

    let pos_test4 = Position::from_fen(FEN_TEST_POSITION_4).unwrap();

    assert_eq!(count_nodes(&mg, &pos_test4, 1), 44);
    assert_eq!(count_nodes(&mg, &pos_test4, 2), 1486);
    assert_eq!(count_nodes(&mg, &pos_test4, 3), 62379);
    assert_eq!(count_nodes(&mg, &pos_test4, 4), 2103487);
    assert_eq!(count_nodes(&mg, &pos_test4, 5), 89941194);

    let pos_test5 = Position::from_fen(FEN_TEST_POSITION_5).unwrap();

    assert_eq!(count_nodes(&mg, &pos_test5, 1), 46);
    assert_eq!(count_nodes(&mg, &pos_test5, 2), 2079);
    assert_eq!(count_nodes(&mg, &pos_test5, 3), 89890);
    assert_eq!(count_nodes(&mg, &pos_test5, 4), 3894594);
    assert_eq!(count_nodes(&mg, &pos_test5, 5), 164075551);
}
