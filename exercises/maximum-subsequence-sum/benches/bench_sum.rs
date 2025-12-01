use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use maximum_subsequence_sum::{max_sum_cubic, max_sum_quadratic, max_sum_nlogn, max_sum_linear};

fn criterion_benchmark(c: &mut Criterion) {
    let r = [1, 2, 3, -100, 4, 5];
    c.bench_function("blah", |b| b.iter(|| max_sum_cubic(&r)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);