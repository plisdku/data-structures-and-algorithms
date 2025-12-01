use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use maximum_subsequence_sum::{
    generate_random_integers, max_sub_sum_cubic, max_sub_sum_linear, max_sub_sum_nlogn,
    max_sub_sum_quadratic,
};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    // let r = [1, 2, 3, -100, 4, 5];

    // Single group to produce comparison plots in the report
    let mut group = c.benchmark_group("comparison");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for &size in [10usize, 100, 1000, 10000].iter() {
        group.throughput(criterion::Throughput::Elements(size as u64));

        let r = generate_random_integers(size, -10, 10);

        // Benchmark each algorithm with a distinct BenchmarkId; Criterion will
        // place them together in the group's charts so you can compare growth.
        group.bench_with_input(BenchmarkId::new("cubic", size), &r, |b, r| {
            b.iter(|| max_sub_sum_cubic(black_box(r)));
        });

        group.bench_with_input(BenchmarkId::new("quadratic", size), &r, |b, r| {
            b.iter(|| max_sub_sum_quadratic(black_box(r)));
        });

        group.bench_with_input(BenchmarkId::new("nlogn", size), &r, |b, r| {
            b.iter(|| max_sub_sum_nlogn(black_box(r)));
        });

        group.bench_with_input(BenchmarkId::new("linear", size), &r, |b, r| {
            b.iter(|| max_sub_sum_linear(black_box(r)));
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
