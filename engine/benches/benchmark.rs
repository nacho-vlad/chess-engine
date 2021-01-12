
use criterion::{criterion_group, criterion_main, Criterion};
use engine::*;
use chess::Chessboard;

pub fn negamax_starting_bench(c: &mut Criterion) {
    let chessboard = Chessboard::starting();
    c.bench_function("depth 3", |b| b.iter(|| best_move(&chessboard, 3)));
    c.bench_function("depth 5", |b| b.iter(|| best_move(&chessboard, 5)));
    let mut group = c.benchmark_group("smaller-sample-size"); 
    group.sample_size(10);
    group.bench_function("depth 8", |b| b.iter(|| best_move(&chessboard, 8)));
    group.finish();
}

criterion_group!(benches, negamax_starting_bench);
criterion_main!(benches);
