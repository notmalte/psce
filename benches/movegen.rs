use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psce::engine::{
    movegen::MoveGen,
    position::{
        Position, FEN_TEST_POSITION_1, FEN_TEST_POSITION_2, FEN_TEST_POSITION_3,
        FEN_TEST_POSITION_4, FEN_TEST_POSITION_5,
    },
};

fn movegen_benchmark(c: &mut Criterion) {
    let mg = MoveGen::new();

    let positions = [
        Position::initial(),
        Position::from_fen(FEN_TEST_POSITION_1).unwrap(),
        Position::from_fen(FEN_TEST_POSITION_2).unwrap(),
        Position::from_fen(FEN_TEST_POSITION_3).unwrap(),
        Position::from_fen(FEN_TEST_POSITION_4).unwrap(),
        Position::from_fen(FEN_TEST_POSITION_5).unwrap(),
    ];

    c.bench_function("movegen pawn", |b| {
        b.iter(|| {
            for pos in &positions {
                black_box(mg.pawn().generate_pseudo_legal_moves(black_box(pos)));
            }
        });
    });

    c.bench_function("movegen knight", |b| {
        b.iter(|| {
            for pos in &positions {
                black_box(mg.knight().generate_pseudo_legal_moves(black_box(pos)));
            }
        });
    });

    c.bench_function("movegen bishop", |b| {
        b.iter(|| {
            for pos in &positions {
                black_box(mg.bishop().generate_pseudo_legal_moves(black_box(pos)));
            }
        });
    });

    c.bench_function("movegen rook", |b| {
        b.iter(|| {
            for pos in &positions {
                black_box(mg.rook().generate_pseudo_legal_moves(black_box(pos)));
            }
        });
    });

    c.bench_function("movegen queen", |b| {
        b.iter(|| {
            for pos in &positions {
                black_box(mg.queen().generate_pseudo_legal_moves(black_box(pos)));
            }
        });
    });

    c.bench_function("movegen king", |b| {
        b.iter(|| {
            for pos in &positions {
                black_box(mg.king().generate_pseudo_legal_moves(black_box(pos), &mg));
            }
        });
    });

    c.bench_function("movegen all", |b| {
        b.iter(|| {
            for pos in &positions {
                black_box(mg.generate_pseudo_legal_moves(black_box(pos)));
            }
        });
    });
}

criterion_group!(benches, movegen_benchmark);
criterion_main!(benches);
