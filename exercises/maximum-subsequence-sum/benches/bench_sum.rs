use criterion::{criterion_group, criterion_main, Criterion};
use maximum_subsequence_sum::{
    max_sub_sum_cubic, max_sub_sum_linear, max_sub_sum_nlogn, max_sub_sum_quadratic,
};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let r = [1, 2, 3, -100, 4, 5];
    c.bench_function("blah", |b| b.iter(|| max_sub_sum_cubic(&r)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
